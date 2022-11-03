/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use actix::{Actor, ActorContext, Context, Handler, Message, Supervised, Supervisor};

#[actix_web::test]
async fn test_supervisor() {
    static START_COUNT: AtomicUsize = AtomicUsize::new(0);
    static RESTART_COUNT: AtomicUsize = AtomicUsize::new(0);
    static RECEIVED_MESSAGE_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct SupervisedActor;

    impl Actor for SupervisedActor {
        type Context = Context<Self>;

        fn started(&mut self, _ctx: &mut Self::Context) {
            // println!("started");
            START_COUNT.fetch_add(1, SeqCst);
        }
    }

    impl Supervised for SupervisedActor {
        fn restarting(&mut self, _ctx: &mut <Self as Actor>::Context) {
            // println!("restarting");
            RESTART_COUNT.fetch_add(1, SeqCst);
        }
    }

    #[derive(Message)]
    #[rtype(result = "()")]
    struct DoSupervisedRestart;

    impl Handler<DoSupervisedRestart> for SupervisedActor {
        type Result = ();

        fn handle(&mut self, _msg: DoSupervisedRestart, ctx: &mut Self::Context) -> Self::Result {
            RECEIVED_MESSAGE_COUNT.fetch_add(1, SeqCst);
            // println!("stop ctx");
            // panics are panics and don't actually invoke graceful restarts by the Supervisor!

            ctx.stop();
        }
    }

    let actor = Supervisor::start(move |_| SupervisedActor);
    actor.send(DoSupervisedRestart).await.expect("Expecting nothing");
    actor.send(DoSupervisedRestart).await.expect("Expecting nothing");

    assert_eq!(START_COUNT.load(SeqCst), 3, "initial + 2 restarts");
    assert_eq!(RESTART_COUNT.load(SeqCst), 2, "2 restarts");
    assert_eq!(RECEIVED_MESSAGE_COUNT.load(SeqCst), 2, "2 received messages");
}
