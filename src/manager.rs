use crate::args::Args;
use crate::args::Command;
use std::process;
use itertools::Itertools;
use std::convert::From;
use anyhow::Result;
use io::stdin;
use std::io;
use std::fmt;
use str_distance::{DistanceMetric, Jaccard};
use regex::Regex;

pub struct Manager {
    args: Args
}
static KUB_CTL: &str = "kubectl -s https://127.0.0.1:6443 --certificate-authority=/srv/kubernetes/ca.pem --client-certificate=/srv/kubernetes/admin.pem  --client-key=/srv/kubernetes/admin-key.pem";

// PodInfo with kubectl get po -owide
#[derive(Debug)]
pub struct PodInfo {
    name: String,
    ready: String,
    status: String,
    restarts: String,
    age: String,
    ip: String,
    node: String,
    nominated_node: String,
    readiness_gates: String,
}

impl From<(&str, &str, &str, &str, &str, &str, &str, &str, &str)> for PodInfo {
    fn from(t: (&str, &str, &str, &str, &str, &str, &str, &str, &str)) -> PodInfo {
        PodInfo {
            name: t.0.to_string(),
            ready: t.1.to_string(),
            status: t.2.to_string(),
            restarts: t.3.to_string(),
            age: t.4.to_string(),
            ip: t.5.to_string(),
            node: t.6.to_string(),
            nominated_node: t.7.to_string(),
            readiness_gates: t.8.to_string(),
        }
    }
}

impl fmt::Display for PodInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.name, self.ready, self.status, self.restarts, self.age, self.ip, self.node, self.nominated_node, self.readiness_gates)
    }
}

impl Manager {
    pub fn new(args: Args) -> Self {
        Manager { args }
    }

    pub fn run(&self) -> Result<String> {
        let command = &self.args.cmd.clone().unwrap();
        let kub_commands = self.get_kub_command(command);
        let mut command_output = Vec::new();
        for kub_command in kub_commands {
            log::info!("{}", kub_command);
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(kub_command)
                .output().expect("failed to execute cmd");
            command_output.push(String::from_utf8_lossy(&output.stdout).to_string());
        }
        Ok(command_output.join("\n"))
    }

    fn get_kub_command(&self, command: &Command) -> Vec<String> {
        let get_pod_name = || -> &str {
            match command {
                Command::DELETE { name } => name,
                Command::DESCRIBE {name} => name,
                Command::IMAGE {name} => name,
                Command::CONTAINER {name} => name,
                Command::LOG {name} => name,
            }
        };
        let pod_name_slice = get_pod_name();
        // if RBL_SOPHON_ALIAS is set to anything, use sophon alias (to unset it use unset)
        let insert_middle_name = self.args.middle.is_some();
        let pod_name_slice = if insert_middle_name {
            let middle_name = self.args.middle.as_ref().unwrap();
            filled_with_middle_name(pod_name_slice, &middle_name[..])
        } else {
            pod_name_slice.to_string()
        };
        let pod_name_slice = &pod_name_slice;
        let candidate_pods = self.get_candidate_pod(pod_name_slice, false);
        if candidate_pods.len() == 0 {
            log::info!("no such a pod named like {} found!", pod_name_slice);
            log::info!("trying fuzzy match...");
            let candidate_pods_fuzzy = self.get_candidate_pod(pod_name_slice, true);
            if candidate_pods_fuzzy.len() == 0 {
                log::info!("fuzzy match has no results...");
                process::exit(0);
            } else {
                handle_multiple_results(command, candidate_pods_fuzzy)
            }
        }
        else if candidate_pods.len() > 1 {
            log::info!("multiple pods named like {} found!", pod_name_slice);
            log::info!("possible choices:");
            handle_multiple_results(command, candidate_pods)
        }
        else {
            vec![get_kub_command(command, &candidate_pods[0].name[..])]
        }
    }

    fn get_candidate_pod(&self, pod_name_slice: &str, fuzzy_match: bool) -> Vec<PodInfo> {
        let all_pods = self.list_pods();
        if !fuzzy_match {
            all_pods.into_iter().filter(
                |pod_info| pod_info.name.contains(pod_name_slice)
            ).collect()
        } else {
            all_pods.into_iter().sorted_by(
                |a, b|
                    Jaccard::new(1).str_distance(&a.name, pod_name_slice).partial_cmp(
                    &Jaccard::new(1).str_distance(&b.name, pod_name_slice)).unwrap()
                ).take(3).collect()
        }
    }

    fn list_pods(&self) -> Vec<PodInfo> {
        // let test_pod = "
        //     sophon-apimanager-sophon2-58f4b7965-n99hz                      1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-approval-sophon2-7748d4b87b-rt8zr                       1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-audit-sophon2-654889f8c-g8xjc                           1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-base-sophon2-557b9f49d4-xf95j                           1/1     Running             0          9d      172.26.0.124   kg-node43   <none>           <none>
        //     sophon-gateway-sophon2-6dbf875495-dckc4                        1/1     Running             5          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-jobmanager-sophon2-5f4df546f6-pld27                     1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-kg-sophon2-bf9769d97-4hqgv                              1/1     Running             0          56m     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-notebook-sophon2-57f5c77786-8lpkw                       1/1     Running             0          20h     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-notification-sophon2-6bc6b754ff-59nbc                   1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-resource-sophon2-5fc7f9dcb7-j7srl                       1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-retrieve-sophon2-f5789bdb4-r7qcg                        1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-session-sophon2-58cd56dbf9-slcjp                        1/1     Running             4          10d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-share-sophon2-7b5795b4d4-gcqwv                          1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        //     sophon-ui-sophon2-79c997dd8c-vkths                             1/1     Running             1          9d      172.26.0.124   kg-node43   <none>           <none>
        //     sophon-user-sophon2-6586dd74c4-r4ndp                           1/1     Running             4          12d     172.26.0.124   kg-node43   <none>           <none>
        // ";
        // let kub_info: Vec<PodInfo> = test_pod.trim().split("\n").map(convert_to_kub_info).collect();
        // kub_info
        let output = std::process::Command::new("sh")
                     .arg("-c")
                     .arg(format!("{} get po -owide | tail -n+2", KUB_CTL))
                     .output()
                     .expect("failed to execute kubectl get po");
        String::from_utf8_lossy(&output.stdout).to_string().trim().split("\n").map(convert_to_kub_info).collect()
    }
}

fn convert_to_kub_info(s: &str) -> PodInfo {
    let kub_output : (&str, &str, &str, &str, &str, &str, &str, &str, &str) = s.split_whitespace().collect_tuple().unwrap();
    let pod_info: PodInfo = kub_output.into();
    pod_info
}

fn handle_multiple_results(cmd: &Command, candidate_pods: Vec<PodInfo>) -> Vec<String> {
    // list three choices
    let choices = ["a", "b", "c"];
    for (x, y) in choices.iter().zip(candidate_pods.iter()) {
        log::info!{"{}: {}", x, y};
    }
    log::info!("d: apply to all");
    log::info!("type your choice...");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input_choice = &input.trim().to_lowercase()[..];
    match input_choice {
        "a" => vec![get_kub_command(cmd, &candidate_pods[0].name[..])],
        "b" => vec![get_kub_command(cmd, &candidate_pods[1].name[..])],
        "c" => {
            if candidate_pods.len() > 2 {
                vec![get_kub_command(cmd, &candidate_pods[2].name[..])]
            } else {
                log::error!("no such a choice {}", input_choice);
                process::exit(1)
            }
        }
        "d" => {
            let mut kub_cmds = Vec::new();
            for candidate_pod in candidate_pods {
                kub_cmds.push(get_kub_command(cmd, &candidate_pod.name[..]));
            }
            kub_cmds
        }
        _ => {
            log::error!("no such a choice {}", input_choice);
            process::exit(1)
        }
    }
}

fn get_kub_command(command: &Command, pod_name: &str) -> String {
    match command {
        Command::DELETE {name: _} => format!("{} delete po {}", KUB_CTL, pod_name),
        Command::DESCRIBE {name: _} => format!("{} describe po {}", KUB_CTL, pod_name),
        Command::LOG {name: _} => format!("{} logs {}", KUB_CTL, pod_name),
        Command::IMAGE {name: _} => format!("{} describe po {} | grep Image", KUB_CTL, pod_name),
        Command::CONTAINER {name: _} => format!("{} describe po {} | grep container", KUB_CTL, pod_name),
    }
}


// if the input pod name is a component followed a version number, e.g. kg2,
// can be converted to kg-sophon2 with `middle` name "-sophon"
// this function is activated when `middle` option is set
fn filled_with_middle_name(pod_name: &str, middle_name: &str) -> String {
    let sophon_reg = Regex::new(r"(.*?)(\d*)$").unwrap();
    if sophon_reg.is_match(pod_name) {
        let caps = sophon_reg.captures(pod_name).unwrap();
        format!("{}{}{}", caps.get(1).unwrap().as_str(), middle_name, caps.get(2).unwrap().as_str()).to_string()
    } else {
        pod_name.to_string()
    }
}

#[test]
fn test_insert_middle_name() {
    assert_eq!(filled_with_middle_name("kg2", "-sophon"), "kg-sophon2");
    assert_eq!(filled_with_middle_name("base22", "-sophon"), "base-sophon22");
    assert_eq!(filled_with_middle_name("notebook", "-sophon"), "notebook-sophon");
    assert_eq!(filled_with_middle_name("gk22", "-sophon"), "gk-sophon22");
    assert_eq!(filled_with_middle_name("datanode1", "-hdfs"), "datanode-hdfs1");
    assert_eq!(filled_with_middle_name("2222", "-test"), "-test2222");
    assert_eq!(filled_with_middle_name("s2s22", "-test"), "s2s-test22");
    assert_eq!(filled_with_middle_name("222s22", "-test"), "222s-test22");
    assert_eq!(filled_with_middle_name("s", "-test"), "s-test");
}