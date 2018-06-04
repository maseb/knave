extern crate actix;
extern crate futures;
use actix::prelude::*;
use futures::{future, Future};

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

fn main() {
    let system = System::new("test");
    let addr: Addr<Unsync, _> = MyActor { count: 10 }.start();
    let res = addr.send(Ping(10));
    Arbiter::handle().spawn(
        res.map(|res| {
            println!("RESULT: {}, {}", res == 20, res);
        }).map_err(|_| ()),
    );
    system.run();
}
