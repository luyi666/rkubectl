use std::process;
use anyhow::Result;
use io::stdin;
use std::io;
use crate::args::Args;
use crate::args::Command;

pub struct Manager {
    args: Args
}
static KUB_CTL: &str = "kubectl -s https://127.0.0.1:6443 --certificate-authority=/srv/kubernetes/ca.pem --client-certificate=/srv/kubernetes/admin.pem  --client-key=/srv/kubernetes/admin-key.pem";

impl Manager {
    pub fn new(args: Args) -> Self {
        Manager { args }
    }

    pub fn run(&self) -> Result<String> {
        let command = &self.args.cmd.clone().unwrap();
        let kub_commands = self.get_kub_command(command);
        let mut command_output = Vec::new();
        for kub_command in kub_commands {
            println!("{}", kub_command);
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(kub_command)
                .output().expect("failed to execute cmd");
            command_output.push(String::from_utf8_lossy(&output.stdout).to_string());
        }
        Ok(command_output.join("\n"))
    }

    fn get_kub_command(&self, command: &Command) -> Vec<String> {
        fn _get_kub_command(cmd: &str, pod_name: &str) -> String {
            if cmd == "delete" || cmd == "describe" {
                format!("{} {} po {}", KUB_CTL, cmd, pod_name)
            }
            else if cmd == "owide" {
                format!("{} get po -owide | grep {}", KUB_CTL, pod_name)
            }
            else if cmd == "log" {
                format!("{} logs {}", KUB_CTL, pod_name)
            }
            else {
                format!("{} describe po {} | grep {}", KUB_CTL, pod_name, cmd)
            }
        }
        let (cmd, pod_name_slice ) = match command  {
            Command::DELETE { name } => ("delete", name),
            Command::DESCRIBE {name} => ("describe", name),
            Command::IMAGE {name} => ("Image", name),
            Command::CONTAINER {name} => ("Container", name),
            Command::OWIDE {name} => ("owide", name),
            Command::LOG {name} => ("log", name),
        };
        let candidate_pods = self.get_candidate_pod(pod_name_slice);
        if candidate_pods.len() == 0 {
            println!("no such pod named like {} found!", pod_name_slice);
            process::exit(0);
        }
        else if candidate_pods.len() > 1 {
            println!("multiple pods named like {} found!", pod_name_slice);
            println!("possible choices:");
            // list three choices
            let choices = ["a", "b", "c"];
            for (x, y) in choices.iter().zip(candidate_pods.iter()) {
                println!{"{}: {}", x, y};
            }
            println!("d: apply to all");
            println!("type your choice...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input_choice = &input.trim().to_lowercase()[..];
            match input_choice {
                "a" => vec![_get_kub_command(cmd, &candidate_pods[0][..])],
                "b" => vec![_get_kub_command(cmd, &candidate_pods[1][..])],
                "c" => {
                    if candidate_pods.len() > 2 {
                        vec![_get_kub_command(cmd, &candidate_pods[2][..])]
                    } else {
                        eprint!("no such choice {}", input_choice);
                        process::exit(1)
                    }
                }
                "d" => {
                    let mut kub_cmds = Vec::new();
                    for candidate_pod in candidate_pods {
                        kub_cmds.push(_get_kub_command(cmd, &candidate_pod[..]));
                    }
                    kub_cmds
                }
                _ => {
                    eprint!("no such a choice {}", input_choice);
                    process::exit(1)
                }
            }
        }
        else {
            vec![_get_kub_command(cmd, &candidate_pods[0][..])]
        }
    }

    fn get_candidate_pod(&self, pod_name_slice: &str) -> Vec<String> {
        let all_pods = self.list_pods();
        let mut candidate_pods = Vec::new();
        for pod in all_pods {
            if pod.contains(pod_name_slice) {
                candidate_pods.push(pod.trim().to_string());
            }
        }
        candidate_pods
    }

    fn list_pods(&self) -> Vec<String> {
        // let test_pod = "sophon-apimanager-sophon2-58f4b7965-n99hz
        // sophon-approval-sophon2-7748d4b87b-rt8zr
        // sophon-audit-sophon2-654889f8c-g8xjc
        // sophon-base-sophon2-557b9f49d4-xf95j
        // sophon-gateway-sophon2-6dbf875495-dckc4
        // sophon-jobmanager-sophon2-5f4df546f6-pld27
        // sophon-kg-sophon2-bf9769d97-nvmnb
        // sophon-notebook-sophon2-57f5c77786-sm2nm
        // sophon-notification-sophon2-6bc6b754ff-59nbc
        // sophon-resource-sophon2-5fc7f9dcb7-j7srl
        // sophon-retrieve-sophon2-f5789bdb4-r7qcg
        // sophon-session-sophon2-58cd56dbf9-slcjp
        // sophon-share-sophon2-7b5795b4d4-gcqwv
        // sophon-ui-sophon2-79c997dd8c-vkths
        // sophon-user-sophon2-6586dd74c4-r4ndp";
        // test_pod.split("\n").map(String::from).collect()
        let output = std::process::Command::new("sh")
                     .arg("-c")
                     .arg(format!("{} get po | awk '{{print $1}}'", KUB_CTL))
                     .output()
                     .expect("failed to execute kubectl get po");
        String::from_utf8_lossy(&output.stdout).to_string().split("\n").map(String::from).collect()
    }
}