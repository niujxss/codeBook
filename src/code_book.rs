
use anyhow::{ensure, Context, Result};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use rand_core::RngCore;

use serde::{Serialize, Deserialize};

use std::{fs, io, path::{Path, PathBuf}};
use std::env;

use crate::mat_data::MatData;

// 加密相关类型
type CryptoError = aes_gcm::Error;
type Cipher = Aes256Gcm;


#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct CodeBook{
    data : Vec<MatData>,
    key : Vec<u8>,
}


impl CodeBook {
    const NONCE_LENGTH: usize = 12; // GCM推荐12字节nonce

    pub fn new(key:&Vec<u8>) -> CodeBook {

         CodeBook{
             data:Vec::new(),
             key:key.clone(),
         }
    }

    pub fn load_or_new() ->CodeBook {
        let key = hex::decode("aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899")
            .expect("hex decode error");
        let path = Self::data_path();
        match Self::load_by_path(&key,path) {
            Ok(data) => {
                data
            },
            Err(e) => {
                if let Some(io_err) = e.downcast_ref::<std::io::Error>(){
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        let new_book = CodeBook::new(&key);
                        new_book.save(&key).expect("save error");
                        new_book
                    } else {
                        panic!("io error");
                    }
                } else {
                    panic!("io error");
                }
            },
        }
    }

    pub fn add(&mut self,name:String,passwd:String,notes:String ) {
        let data = MatData::new(name,passwd,notes);
        self.data.push(data);

        self.save(&self.key).unwrap();

    }

    pub fn add_from_file(&mut self,filepath:PathBuf) {

        let codebook = match Self::load_by_path(&self.key,filepath) {
            Ok(data) => {data},
            Err(e) => {println!("{}",e); return;},
        };
        self.extend(codebook);
        self.save(&self.key).expect("save error");
    }

    pub fn showdata(&self) {
        for item in &self.data {
            item.showdata();
        }
    }

    // // 获取存储路径（跨平台） 暂时不使用用，该函数会返回一个标准的软件存储的数据的路径
    // fn data_path() -> Result<PathBuf> {
    //     let dirs = directories::ProjectDirs::from("com", "MyCompany", "SecureData") //根据操作系统不同返回标准目录；
                                                    //三个参数，第一个：作者，通常为公司，组织或个人名 com.example
                                                    //第二个： 应用程序名
                                                    //第三个：项目名
    //         .context("Failed to get project dirs")?;
    //     let data_dir = dirs.data_dir();
    //     std::fs::create_dir_all(data_dir)?;
    //     Ok(data_dir.join("secure.vault"))
    // }

    fn data_path() -> PathBuf {
        // 直接返回当前目录下的文件名
        let exe_dir = env::current_exe()
            .expect("无法获取可执行文件路径")
            .parent()
            .expect("无法获取父目录")
            .to_path_buf();

        exe_dir.join("secure.ck")

    }

    fn init_cipher(key: &[u8]) -> Result<Cipher> {
        ensure!(key.len() == 32,"长度错误，必须为32字节，现在字节数为：{}",key.len());

        let key = Key::<Aes256Gcm>::from_slice(key);//将原始字节转换为AES-256-GCM专用密钥类型
        Ok(Aes256Gcm::new(key))
    }


    pub fn load_by_path(key: &[u8],path:PathBuf) -> Result<Self> {
        // let path = Self::data_path();
        // println!("{}",path.display());
        println!("filepath is {}",path.display());
        //let data = fs::read(path).context("读取文件失败")?;
        let data = fs::read(path)?;

        if data.len() < Self::NONCE_LENGTH { //数据长度不对，报错返回
            anyhow::bail!("文件长度不对！！");
        }

        let (nonce_bytes, encrypted_data) = data.split_at(Self::NONCE_LENGTH);

        //初始化加密器
        let cipher = Self::init_cipher(key)?; //初始化AES
        let nonce = Nonce::from_slice(nonce_bytes);  //将原始数据转换成nonce数据

        // 解密数据
        let decrypted = cipher.decrypt(nonce, encrypted_data)
            .context("Decryption failed")?;

        //反序列化
        bincode::deserialize(&decrypted)
            .context("Failed to deserialize data")

    }

    pub fn save(&self, key: &[u8]) -> Result<()> {
        let path = Self::data_path();
        // 序列化为二进制
        let serialized = bincode::serialize(self)?;
        // 生成随机nonce
        let mut nonce = [0u8; Self::NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce);
        // 初始化加密器
        let cipher = Self::init_cipher(key)?;
        let nonce = Nonce::from_slice(&nonce);
        // 加密数据
        let encrypted_data = cipher.encrypt(nonce, serialized.as_ref())
            .context("Encryption failed")?;
        // 合并nonce和加密数据
        let mut final_data = nonce.to_vec();
        final_data.extend(encrypted_data);
        // 写入文件
        fs::write(&path, &final_data) //自动创建文件，覆盖写
            .context("Failed to write encrypted data")?;
        Ok(())
    }

    fn extend(&mut self, data:CodeBook) {
        self.data.extend(data.data);
    }

}