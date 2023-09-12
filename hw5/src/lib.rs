#![feature(impl_trait_in_assoc_type)]
use std::{collections::HashMap, sync::Mutex};

use pilota::FastStr;
use volo_gen::volo::example::{RCommand, GetItemResponse, GetItemRequest};
pub struct KV{
    pub map: Mutex<HashMap<String, String>>,
}

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for KV {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_gen::volo::example::GetItemRequest,
    ) -> core::result::Result<volo_gen::volo::example::GetItemResponse, volo_thrift::AnyhowError>
    {
        match _req.cmd{
            RCommand::Get=>{
                if let Some(arg) = _req.args {
                    if arg.len() != 1 {
                        Ok(GetItemResponse { 
                            ok: false, 
                            data: Some(FastStr::from(format!(
                                "Error! Expected str mount: 1, got {}", 
                                arg.len()
                            )))
                        }) 
                    } else {
                        if let Some(value) = self.map.lock().unwrap().get(&arg[0].to_string()) {
                            Ok(GetItemResponse { 
                                ok: true, 
                                data: Some(FastStr::from(value.to_string()))
                            })
                        } else {
                            Ok(GetItemResponse { 
                                ok: false, 
                                data: Some(FastStr::from("Error,not found"))
                            })
                        }
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: false, 
                        data: Some(FastStr::from("Args Error!"))
                    })
                }
            }
            RCommand::Set=>{
                if let Some(arg) = _req.args {
                    if arg.len() != 2 {
                        Ok(GetItemResponse { 
                            ok: false,
                            data: Some(FastStr::from(format!(
                                "Args Error! Expected str mount:2, got {}", 
                                arg.len()
                            ))) 
                        })
                    } else {
                        let (key, value) = (&arg[0], &arg[1]);
                        if self.map.lock().unwrap().insert(key.to_string(), value.to_string()).is_some() {
                            Ok(GetItemResponse { 
                                ok: true,
                                data: Some(FastStr::from("Value Updated!")) 
                            })
                        } else {
                            Ok(GetItemResponse { 
                                ok: true,
                                data: Some(FastStr::from("Set Success!")) 
                            })
                        }
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: false,
                        data: Some(FastStr::from("Args Error!")) 
                    })
                }
            }
            RCommand::Ping=>{
                if let Some(arg) = _req.args {
                  if arg.len()<1{
                    Ok(GetItemResponse{ok:true,data:Some(FastStr::from("PONG"))})
                  }
                  else{Ok(GetItemResponse{ok:true,data:Some(FastStr::from("WHAT?"))})}
                }
                else{
                    Ok(GetItemResponse { 
                        ok: false,
                        data: Some(FastStr::from("Args Error!")) 
                    })
                }
            }
            RCommand::Del=>{
                if let Some(arg) = _req.args {
                    if arg.len() < 1 {
                        Ok(GetItemResponse { 
                            ok: false,
                            data: Some(FastStr::from(format!(
                                "Args Error! Expected str mount:1, got {}", 
                                arg.len()
                            ))) 
                        })
                    } else {
                        let mut count = 0;
                        for key in arg {
                            count += self.map.try_lock().unwrap().remove(&(key.to_string())).is_some() as i32;
                        }
                        Ok(GetItemResponse { 
                            ok: true,
                            data: Some(FastStr::from(format!("Del num:{}", count))) 
                        })
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: false,
                        data: Some(FastStr::from("Args Error!")) 
                    })
                }
            }
            RCommand::Publish=>{
                Ok(GetItemResponse { 
                    ok: false,
                    data: Some(FastStr::from("Not impl!")) 
                })
            }
            RCommand::Subscribe=>{
                Ok(GetItemResponse { 
                    ok: false,
                    data: Some(FastStr::from("Not impl!")) 
                })
            }
            _ =>{
                Ok(GetItemResponse{ok:false, data:Some(FastStr::from("Not found!"))})
            }
        }
    }

}




pub struct LogLayer;

impl<KV> volo::Layer<KV> for LogLayer {
    type Service = LogService<KV>;

    fn layer(self, inner: KV) -> Self::Service {
        LogService(inner)
    }
}


#[derive(Clone)]
pub struct LogService<KV>(KV);

#[volo::service]
impl<Cx, Req, KV> volo::Service<Cx, Req> for LogService<KV>
where
    Req: std::fmt::Debug + Send + 'static,
    KV: Send + 'static + volo::Service<Cx, Req> + Sync,
    KV::Response: std::fmt::Debug,
    KV::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<KV::Response, KV::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}
pub struct FilterLayer;

impl<KV> volo::Layer<KV> for FilterLayer {
    type Service = FilterService<KV>;

    fn layer(self, inner: KV) -> Self::Service {
        FilterService(inner)
    }
}
#[derive(Clone)]
pub struct FilterService<KV>(KV);
#[volo::service]
impl<Cx, Req, KV> volo::Service<Cx, Req> for FilterService<KV>
where
    Req: std::fmt::Debug + Send + 'static,
    KV: Send + 'static + volo::Service<Cx, Req> + Sync,
    KV::Response: std::fmt::Debug,
    KV::Error: std::fmt::Debug,
        anyhow::Error: Into<KV::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<KV::Response, KV::Error> {
        let info = format!("{:?}", req);
        if info.contains("wxz") {
            return Err(anyhow::anyhow!("[wxz] is not allowed").into());
        }
        self.0.call(cx, req).await
    }
}
