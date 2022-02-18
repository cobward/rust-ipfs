use futures::join;
use ipfs::{Ipfs, IpfsOptions, IpfsPath, TestTypes};
use libipld::ipld;
use tokio::task;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize the repo and start a daemon
    let (ipfs, fut): (Ipfs<TestTypes>, _) = IpfsOptions::inmemory_with_generated_keys()
        .create_uninitialised_ipfs()
        .unwrap()
        .start()
        .await
        .unwrap();
    task::spawn(fut);

    // Create a DAG
    let f1 = ipfs.put_dag(ipld!("block1"));
    let f2 = ipfs.put_dag(ipld!("block2"));
    let (res1, res2) = join!(f1, f2);
    let root = ipld!([res1.unwrap(), res2.unwrap()]);
    let cid = ipfs.put_dag(root).await.unwrap();
    let path = IpfsPath::from(cid);

    // Query the DAG
    let path1 = path.sub_path("0").unwrap();
    let path2 = path.sub_path("1").unwrap();
    let f1 = ipfs.get_dag(path1);
    let f2 = ipfs.get_dag(path2);
    let (res1, res2) = join!(f1, f2);
    println!("Received block with contents: {:?}", res1.unwrap());
    println!("Received block with contents: {:?}", res2.unwrap());

    // Exit
    ipfs.exit_daemon().await;
}
