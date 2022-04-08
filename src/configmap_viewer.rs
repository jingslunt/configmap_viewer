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
    let args = std::env::args().nth(1).expect("Did not provide  argument");

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
                    if name == &args {
                        if let Some(data) = yaml_data {
                            let v = serde_json::to_value(data)?;
                            println!("{}", v);
                        }
                    }
                }

            }
        }
    }
    Ok(())
}
