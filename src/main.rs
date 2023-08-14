use std::{sync::{Arc, Mutex, mpsc::{Sender, channel}}, net::TcpListener, thread, io::{Write, Read}};

#[derive(Copy, Clone)]
struct M;

fn server() -> (Sender<M>, Arc<Mutex<Vec<Sender<M>>>>) {
    let (tx, rx) = channel::<M>();
    let connected_clients: Arc<Mutex<Vec<Sender<M>>>> = Arc::new(Mutex::new(Vec::new()));

    let clients = connected_clients.clone();

    thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            let clients = clients.lock().unwrap();
            for client in clients.iter() {
                client.send(msg.clone()).unwrap();
            }
        }
    });
    (tx, connected_clients)
}

fn main() {

    let (send, clients) = server();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for client in listener.incoming() {
        match client {
            Ok(client) => {
                let reader_client = Arc::new(client);
                let writer_client = reader_client.clone();

                let send_to_server = send.clone();

                thread::spawn(move || {
                    let reader = (&mut &(*reader_client));
                    let _ = send_to_server.send(M);
                    let _ = reader.read(vec![].as_mut_slice());
                });

                let (send_to_socket, receive_from_sever) = channel::<M>();

                clients.lock().unwrap().push(send_to_socket);

                thread::spawn(move || {
                    let msg = receive_from_sever.recv().unwrap();
                    (&mut &(*writer_client)).write_all(vec![].as_mut_slice());
                });

                // thread::spawn(move || {
                //     thread::scope(|s| {
                //         s.spawn(|| {
                //             (&client).read(vec![].as_mut_slice());
                //         });
                //         s.spawn(|| {
                //             (&client).write_all(vec![].as_mut_slice());
                //         });
                //     });
                // });
            
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    // Ok(());
}
