use std::{collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;

use crate::{db::db::Redis, interface::command_type::CommandType, session::session::Session, tools::resp::RespValue, RedisConfig};
use crate::interface::command_strategy::CommandStrategy;
/*
 * FlushAll 命令
 */
pub struct FlushAllCommand {}

impl CommandStrategy for FlushAllCommand {
    fn execute(
        &self,
        stream: Option<&mut TcpStream>,
        _fragments: &[&str],
        redis: &Arc<Mutex<Redis>>,
        _redis_config: &Arc<RedisConfig>,
        _sessions: &Arc<Mutex<HashMap<String, Session>>>,
        _session_id: &str
    ) {
        let mut redis_ref = redis.lock().unwrap();
        
        redis_ref.flush_all();
        
        if let Some(stream) = stream { 
            let response_bytes = &RespValue::Ok.to_bytes();
            match stream.write(response_bytes) {
                Ok(_bytes_written) => {
                    // Response successful
                },
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                },
            };
        }
    }
        
    fn command_type(&self) -> crate::interface::command_type::CommandType {
        CommandType::Write
    }
}