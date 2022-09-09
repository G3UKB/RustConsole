
    use std::net::UdpSocket;
    use std::time::Duration;
    use std::sync::Arc;

    use get_if_addrs;
    use socket2;

    pub struct Sockdata{
        sock2 : Arc<socket2::Socket>,
    }
    
    impl Sockdata {
        pub fn new() -> Sockdata {
            let sock = Self::udp_open_bc_socket();
            Sockdata {  
                sock2 : Arc::new(socket2::Socket::from (sock)),
            }
        }

        pub fn udp_socket_ann(&mut self) {
            println!("UDP Socket");
        }

        pub fn udp_revert_socket(&mut self) {
            self.sock2.set_broadcast(false).expect("set_broadcast call failed");
            self.sock2.set_read_timeout(Some(Duration::from_millis(10))).expect("set_read_timeout call failed");
            // Set buffer sizes?
            self.sock2.set_recv_buffer_size(192000);
            self.sock2.set_send_buffer_size(192000);
        }

        pub fn udp_sock_ref(&mut self) -> &socket2::Socket {
            return &self.sock2;
        }

        fn udp_open_bc_socket() -> UdpSocket {
            let sock = UdpSocket::bind(Self::get_ip() + ":" + "10000").expect("couldn't bind to address");
            sock.set_broadcast(true).expect("set_broadcast call failed");
            sock.set_read_timeout(Some(Duration::from_millis(10))).expect("set_read_timeout call failed");
            return sock
        }

        fn get_ip() -> String{
            let iface = get_if_addrs::get_if_addrs().unwrap();
            return iface[0].ip().to_string();
        }
    }