use tungstenite::{Message, connect, stream::MaybeTlsStream, WebSocket, Error};
use std::{collections::HashMap, mem::Discriminant, net::TcpStream as TCPStream};
use tungstenite::Bytes;
use uuid::Uuid;
use crate::encoding::{self as enc, CdcEncoder};


pub enum Request{
    API = 1,
    COMMAND = 2,
    CONFIGURATION = 3,
    CONSOLE = 4,
    DATA_ARRAY = 5,
    DATA_ATTR = 6,
    DATA_INDEX = 7,
    DATA_SHAPE = 8,
    DOC = 9,
    EQUAL = 10,
    EXCEPTION = 11,
    EXIT = 12,
    GET = 13,
    GETATTR = 14,
    FILTER = 15,
    IMPORT = 16,
    INDEX = 17,
    KEY = 18,
    LEN = 19,
    LESS = 20,
    LINE = 21,
    LOG = 22,
    OBJECTTYPES = 23,
    QUERY = 24,
    REGISTER = 25,
    RELEASE = 26,
    REPR = 27,
    RESOURCE_KEY = 28,
    RESOURCE_LEN = 29,
    RESULT = 30,
    RUNAPI = 31,
    SERVICE = 32,
    SETATTR = 33,
    SETENV = 34,
    TEST = 35,
    TOKENS = 36,
    TRANSLATE = 37,
    TYPE_CALL = 38,
    TYPE_CONSTRUCT = 39,
    TYPE_CMP = 40,
    TYPE_DOC = 41,
    TYPE_GETATTR = 42,
    TYPE_GETITEM = 43,
    TYPE_ITER = 44,
    TYPE_LEN = 45,
    TYPE_REPR = 46,
    TYPE_SETATTR = 47,
    TYPE_SETITEM = 48,
    TYPE_STR = 49,

    TEST_0 = 1000,
    TEST_1 = 1001,
    TEST_2 = 1002,
    TEST_3 = 1003,
    TEST_4 = 1004,
    TEST_5 = 1005,

}
pub mod connection{
    pub mod error{
        pub const ABORT: &str = "Tom::GScript::BreakException";
        pub const ATTRIBUTE: &str = "Tom::GScript::AttributeException";
        pub const IMPORT: &str = "Tom::GScript::ImportException";
        pub const INDEX: &str = "Tom::GScript::IndexException";
        pub const PYTHON: &str = "Tom::GScript::PythonException";

    }
    pub(crate) mod reply{
        use tungstenite::Bytes;

        use crate::encoding::CdcValue;

        pub(crate) struct Error{
            pub(crate) error_type: String,
            pub(crate) description: String,
            pub(crate) code: i64,
            pub(crate) log: String,
            pub(crate) value: Bytes,
        }
        
        pub(crate) enum Reply{
            ERROR(Error),
            REPLY(CdcValue),
        }
    }
    
    pub mod attribute{
        pub const TYPE: &str = "type";
        pub const ID: &str = "id";
        pub const INTERPRETER: &str = "interpreter";
        pub const VALUE: &str = "value";
        pub const STATE: &str = "state";
        pub const PARAMS: &str = "params";
        pub const ARGS: &str = "args";
        pub const KWARGS: &str = "kwargs";
        pub const ERROR: &str = "error";
        pub const DESCRIPTION: &str = "description";
        pub const CODE: &str = "code";
        pub const LOG: &str = "log";
        pub const APIKEY: &str = "apikey";
        pub mod types{
            pub const ERROR: &str = "error";
            pub const REQUEST: &str = "request";
            pub const REPLY: &str = "reply";
            pub const CALL: &str = "call";
            pub const RESULT: &str = "result";
            pub const WAIT: &str = "wait";
        }
    }
}
struct UnexcpectedReply{
    expected_type: enc::CdcType,    
    received_type: enc::CdcType,
}
#[derive(Debug)]
pub enum ConnectionError{
    Attribute,
    Import,
    Index,
    Request,
    Break,
}
impl From<connection::reply::Error> for ConnectionError{
    fn from(err: connection::reply::Error) -> Self {
        match err.error_type.as_str(){
            connection::error::ABORT => ConnectionError::Break,
            connection::error::ATTRIBUTE => ConnectionError::Attribute,
            connection::error::IMPORT => ConnectionError::Import,
            connection::error::INDEX => ConnectionError::Index,
            _ => ConnectionError::Request
        }
    }
}
pub struct Conntection {
    socket: WebSocket<MaybeTlsStream<TCPStream>>,
    api_acces_key: String,
    replies: HashMap<Uuid, connection::reply::Reply>,
    encoder: enc::CdcEncoder,
}

impl Conntection {
    pub fn init(uri: &str, api_key: String) -> Result<Self, Error> {
        let (mut socket, response) = connect(uri)?;
        Ok(Self { socket: socket, api_acces_key: api_key, replies: HashMap::new(), encoder: CdcEncoder::new() })
    }

    pub fn register(&mut self, interpreter_id: &str, filename: &str) -> Result<enc::CdcValue, ConnectionError> {
        let mut params = std::collections::HashMap::new();
        params.insert("id".to_string(), enc::CdcValue::STRING(interpreter_id.to_string()));
        params.insert("file".to_string(), enc::CdcValue::STRING(filename.to_string()));
        self.request(Request::REGISTER, params)
    }
    fn send(&mut self, value: enc::CdcValue) -> Result<(), Error> {
        let bytes = Bytes::from(self.encoder.encode(value));
        self.socket.send(Message::Binary(bytes))
    }
    pub fn request(&mut self, command: Request, params: std::collections::HashMap<String, enc::CdcValue>) -> Result<enc::CdcValue, ConnectionError> {
        let request_id = Uuid::new_v4();
        let mut map: std::collections::HashMap<String, enc::CdcValue> = std::collections::HashMap::new();
        map.insert(connection::attribute::TYPE.into(), enc::CdcValue::STRING(connection::attribute::types::REQUEST.into()));
        map.insert(connection::attribute::APIKEY.into(), enc::CdcValue::STRING(self.api_acces_key.clone()));
        map.insert(connection::attribute::ID.into(), enc::CdcValue::STRING(request_id.to_string()));
        map.insert(connection::attribute::VALUE.into(), enc::CdcValue::INTEGER(command as i64));
        map.insert(connection::attribute::PARAMS.into(), enc::CdcValue::MAP(params));
        let _ = self.send(enc::CdcValue::MAP(map)).expect("Could not send the request!");

        while !(self.replies.contains_key(&request_id)){
            let msg = self.socket.read().expect("Couldn't read from the socket!");
            let msg =self.encoder.decode_value(&mut msg.into_data().as_ref()).expect("Couldn't decode the a reply from the server"); 
            let mut msg_dict = msg.expect_map();
            let msg_type = msg_dict.remove(connection::attribute::TYPE).expect("Type missing from msg dict");
            let msg_type = msg_type.expect_string();
            match &msg_type[..] {
                connection::attribute::types::ERROR => {
                    let reply = connection::reply::Error{
                        error_type: msg_dict.remove(connection::attribute::TYPE).expect("Missing type key in error").expect_string(),
                        description: msg_dict.remove(connection::attribute::DESCRIPTION).expect("Missing description key in error").expect_string().clone(),
                        code: msg_dict.remove(connection::attribute::CODE).expect("Missing code key in error").expect_int() as i64,
                        log: msg_dict.remove(connection::attribute::LOG).expect("Missing log key in error").expect_string().clone(),
                        value: Bytes::from(msg_dict.remove(connection::attribute::VALUE).expect("Missing value key in error").expect_blob()),
                    };
                    self.replies.insert(request_id, connection::reply::Reply::ERROR(reply));
                },
                connection::attribute::types::REPLY => {
                    let reply_value = msg_dict.get(connection::attribute::VALUE).expect("Missing value key in reply").clone();
                    self.replies.insert(request_id, connection::reply::Reply::REPLY(reply_value));
                },
                connection::attribute::types::WAIT => {
                    // Ignore wait messages
                },
                connection::attribute::types::CALL => {
                    let func = msg_dict.get(connection::attribute::VALUE).expect("Missing value key in call").clone().expect_callable();
                    let args = msg_dict.get(connection::attribute::ARGS).expect("Missing args key in call").clone().expect_list();
                    let kwargs = msg_dict.get(connection::attribute::KWARGS).expect("Missing kwargs key in call").clone().expect_map();
                    let result = func(args, kwargs);
                    let r = self.send(result);
                    if r.is_err(){
                        panic!("Failed to send call result back to server!");
                    }
                },
                _ => {
                    panic!("Unknown message type received: {}", msg_type);
                }
            }
        }
        let result = self.replies.remove(&request_id).expect("Ended receiving loop before the message was received!");
        match result{
            connection::reply::Reply::ERROR(err) => Err(ConnectionError::from(err)),
            connection::reply::Reply::REPLY(value) => Ok(value),
        }
    }
}