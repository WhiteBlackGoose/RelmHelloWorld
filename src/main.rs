use std::collections::BTreeMap;
use std::time::Duration;

use gtk::prelude::{BoxExt as _, ButtonExt, GtkWindowExt as _, OrientableExt};
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt as _, SimpleComponent, view};
use relm4::{factory, prelude::*};

struct AppModel {
    progresses: BTreeMap<u64, f32>,
    last_pb_id: u64,
    progs_fac: factory::FactoryVecDeque<MyBar>,
}

#[derive(Debug)]
enum AppMsg {
    AddProgressBar,
}

#[derive(Debug)]
enum AppMsgCmd {
    SetProgress(u64, f32),
    DeleteProgressBar(u64),
}

#[relm4::component]
impl Component for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();
    type CommandOutput = AppMsgCmd;

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                set_expand: true,

                gtk::Button {
                    set_label: "Add Pb",
                    connect_clicked => AppMsg::AddProgressBar
                },

                gtk::ScrolledWindow {

                    #[local_ref]
                    progresses -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_expand: true,
                    }
                }
            }
        }
    }

    fn init(
        _counter: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            progresses: Default::default(),
            last_pb_id: Default::default(),
            progs_fac: FactoryVecDeque::builder()
                .launch(gtk::Box::default())
                .detach(),
        };

        let progresses = model.progs_fac.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AppMsg::AddProgressBar => {
                for _ in 0..10 {
                    self.last_pb_id += 1;
                    let id = self.last_pb_id;
                    sender.command(move |sender, _shutdown| async move {
                        let count = 1000;
                        let mut int = relm4::tokio::time::interval(Duration::from_millis(10));
                        for i in 0..count {
                            sender
                                .send(AppMsgCmd::SetProgress(id, i as f32 / count as f32))
                                .unwrap();
                            int.tick().await;
                        }
                        sender.send(AppMsgCmd::DeleteProgressBar(id)).unwrap();
                    });
                }
            }
        }
        let mut g = self.progs_fac.guard();
        g.clear();
        for (_, val) in &self.progresses {
            g.push_back(MyBar { value: *val as f64 });
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppMsgCmd::SetProgress(id, val) => {
                self.progresses.insert(id, val);
            }
            AppMsgCmd::DeleteProgressBar(id) => {
                self.progresses.remove(&id);
            }
        }
        let mut g = self.progs_fac.guard();
        g.clear();
        for (_, val) in &self.progresses {
            g.push_back(MyBar { value: *val as f64 });
        }
    }
}

struct MyBar {
    value: f64,
}

#[relm4::factory]
impl FactoryComponent for MyBar {
    type Init = Self;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            gtk::ProgressBar {
                #[watch]
                set_fraction: self.value,
                set_margin_all: 10
            }
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        init
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
