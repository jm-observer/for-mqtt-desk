use crate::data::common::{PublicMsgInput, SubscribeInput};
use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use druid::im::Vector;
use std::sync::mpsc::Receiver;

pub fn deal_event(event_sink: druid::ExtEventSink, rx: Receiver<AppEvent>) {
    loop {
        if let Ok(event) = rx.recv() {
            event_sink.add_idle_callback(move |data: &mut AppData| match event {
                AppEvent::Connect(id) => {
                    if let Some(status) = data.tab_statuses.get_mut(&id) {
                        status.try_connect = true;
                    }
                    if data.subscribe_hises.get_mut(&id).is_none() {
                        data.subscribe_hises.insert(id, Vector::new());
                    }
                    data.subscribe_topics.insert(id, Vector::new());
                    data.msgs.insert(id, Vector::new());
                    data.subscribe_ing
                        .insert(id, SubscribeInput::default().into());
                    data.public_ing.insert(id, PublicMsgInput::default().into());
                }
            });
        } else {
            break;
        }
    }
}

// pub subscribe_hises: HashMap<usize, Vector<Arc<SubscribeHis>>>,
// pub subscribe_topics: HashMap<usize, Vector<Arc<SubscribeTopic>>>,
// pub msgs: HashMap<usize, Vector<Arc<Msg>>>,
// pub subscribe_ing: HashMap<usize, Arc<SubscribeInput>>,
// pub public_ing: HashMap<usize, Arc<PublicMsgInput>>,
