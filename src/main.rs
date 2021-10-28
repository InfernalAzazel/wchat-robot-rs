
use std::{fs::File, io::Write};

use inline_python::{python, Context};
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn friend_message_comming(msg: &str) {
	println!("收到好友消息:{}", msg);
}

#[pyfunction]
fn group_message_comming(msg: &str) {
	println!("收到群消息:{}", msg);
}

/// 二维码回调函数参考: https://github.com/why2lyj/ItChat-UOS/blob/master/itchat/components/login.py
#[pyfunction]
fn qr_callback(uuid:&str, status:&str, qrcode:&[u8]) {
    if let Ok(mut file) = File::create("code.png"){
        if let Ok(()) = file.write_all(qrcode){
            println!("二维码已保存 uuid={}, status={}", uuid, status);
            return;
        }
    }
	println!("二维码保存失败！");
}

pub fn run() {
	let c = Context::new();
    c.add_wrapped(wrap_pyfunction!(friend_message_comming));
    c.add_wrapped(wrap_pyfunction!(group_message_comming));
    c.add_wrapped(wrap_pyfunction!(qr_callback));

	c.run(python! {
		import itchat, time, json
		from itchat.content import *

		@itchat.msg_register([TEXT, MAP, CARD, NOTE, SHARING], isFriendChat=True)
		def accept_friend_chat(msg):
			print(msg.user.NickName, msg.text, time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(msg.CreateTime)))
        
		@itchat.msg_register([TEXT, MAP, CARD, NOTE, SHARING], isGroupChat=True)
		def accept_group_chat(msg):
			#print(json.dumps(msg))
			print(msg.ActualNickName, msg.text, time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(msg.CreateTime)))
		
		#itchat.auto_login(True, enableCmdQR=2, qrCallback=qr_callback)
		itchat.auto_login(True, enableCmdQR=2)
		itchat.run(True)
	});
}

fn main() {
	run()
}