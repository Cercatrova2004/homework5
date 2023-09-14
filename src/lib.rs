//volo
#![feature(impl_trait_in_assoc_type)]

use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast::Sender;

pub struct LogLayer;

pub struct S {
	kv: Mutex<HashMap<String, String>>,
	pub channels: Mutex<HashMap<String, Sender<String>>>
}

impl S {
	pub fn new() -> S {
		S {kv: Mutex::new(HashMap::new()), channels: Mutex::new(HashMap::new())}
	}
}

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

#[derive(Clone)]
	pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
	Req: std::fmt::Debug + Send + 'static,
	S: Send + 'static + volo::Service<Cx, Req> + Sync,	
	S::Response: std::fmt::Debug,
	S::Error: std::fmt::Debug,
	Cx: Send + 'static,
{
	async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
		let now = std::time::Instant::now();
		tracing::debug!("Received request {:?}", &req);
		let resp = self.0.call(cx, req).await;
		tracing::debug!("Sent response {:?}", &resp);
		tracing::info!("Request took {}ms", now.elapsed().as_millis());
		resp
	}
}

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
	async fn get_item(&self, _req: volo_gen::volo::example::GetItemRequest) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>{
		let mut resp = volo_gen::volo::example::GetItemResponse{
			op: " ".into(), 
			key: " ".into(), 
			val: " ".into(), 
			status: false
		};
		println!("Received!");
		let k = _req.key.to_string();
		let v = _req.val.to_string();
		match _req.op.as_str() {
			"set" => {
				resp.op = "set".to_string().into();
				let mut flag = 0;
				if self.kv.lock().unwrap().get(&k) == None {
					flag = 1;
				}
				match flag {
					1 => {
						self.kv.lock().unwrap().insert(k, v);
						resp.status = true;
					}
					0 => {
						resp.status = false;
					}
					_ => {
						resp.status = false;
					}
				}
			}
			"get" => {
				resp.op = "get".to_string().into();
				match self.kv.lock().unwrap().get(&k)  {
					None => {
						resp.status = false;
					}
					Some(t) => {
						resp.val = t.clone().into();
						resp.status = true;
					}
				}
			}
			"del" => {
				resp.op = "del".to_string().into();
				match self.kv.lock().unwrap().remove(&k) {
					Some(_t) => {
						resp.status = true;
					}
					None => {
						resp.status = false;
					}
				}
			}
			"ping" => {
				resp.op = "ping".to_string().into();
				resp.status = true;
			}
			_ => {
				panic!("INVALID!");
			}
		}
		println!("Finish!");
		Ok(resp)
	}
}
