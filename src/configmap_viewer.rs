use serde_json;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ListParams},
    runtime::watcher,
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    // 从环境变量中读取NAMESPACE
    let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| "default".into());
    // 读取参数
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}",args.len());
    if args.len() != 3 {
         panic!(
             "please input configmap name and file name.\
         example: {} open-api application.yml",&args[0]
         )
    }
    let cms: Api<ConfigMap> = Api::namespaced(client, &namespace);
    let lp = ListParams::default();

    let mut w = watcher(cms, lp).boxed();
    if let Some(event) = w.try_next().await? {
        if let watcher::Event::Restarted(objs) = event {
            let vecter_iterator = objs.iter() ;
            for elem in vecter_iterator {
                let yaml_name = &elem.metadata.name;
                let yaml_data = &elem.data;
                // 匹配输入的cm名获取对应的configmap内容
                if let Some(name) = yaml_name {
                    let filename = &args[2];
                     // println!("{:?},{:?},{:?}",&name,&args[1],&args[2]);
                    if name == &args[1]  {
                        if let Some(data) = yaml_data {
                            let v = serde_json::to_value(data.get(filename))?;
                            println!("{}", v.to_string().trim());
                        }
                    }
                }

            }
        }
    }
    Ok(())
}

