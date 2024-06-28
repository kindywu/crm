use anyhow::Result;
use crm::User;
use prost::Message;

fn main() -> Result<()> {
    let user = User::new(257, "你", "kindywu@qq.com");
    println!("{user:?}");
    /*
       message User {
        uint64 id = 1;
        string name = 2;
        string email = 3;
        google.protobuf.Timestamp created_at = 4;
       }
    */
    println!("{:?}", user.encode_to_vec());
    //[8, 128, 2] id -> 1000 0001 0000 0010 -> 257
    //[18, 3, 228, 189, 160] name -> (228, 189, 160) -> 你
    //[26, 14, 107(k), 105(i), 110(n), 100(d), 121(y), 119(w), 117(u), 64(@), 113(q), 113(q), 46(.), 99(c), 111(o), 109(m)] email -> kindywu@qq.com
    //[34, 11, 8, 213, 223, 248, 179, 6, 16, 147, 243, 176, 48] -> created_at -> Timestamp { seconds: 1719547893, nanos: 563213765 }
    // 1 0111 1101 1101 0000 1111 1011 0010 1101 1010 0001 1011 0100 1000 0000 0000

    Ok(())
}
