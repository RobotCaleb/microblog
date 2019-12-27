use chrono::{DateTime, Local, Utc};
use serde_derive::{Deserialize, Serialize};
use std::cmp::min;
use yew::{
    format::Json,
    html,
    services::storage::{Area, StorageService},
    Component, ComponentLink, Html, ShouldRender,
};

const BLOG_KEY: &'static str = "yew.microblog.self";
const BLOG_TITLE: &'static str = "My Thoughts";
const PAGE_SIZE: usize = 10;

pub struct Blog {
    storage: StorageService,
    state: BlogState,
}

#[derive(Serialize, Deserialize)]
struct BlogState {
    inflight: BlogEntry,
    entries: Vec<BlogEntry>,
    adding: bool,
    page_size: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BlogEntry {
    title: String,
    body: String,
    time: DateTime<Utc>,
    id: usize,
}

impl BlogEntry {
    fn new() -> BlogEntry {
        BlogEntry {
            title: "".into(),
            body: "".into(),
            time: Utc::now(),
            id: 0,
        }
    }
}

impl Blog {
    fn view_input(&self) -> Html<Blog> {
        let adding = self.state.adding;
        let button_class = if adding {
            "add-button close"
        } else {
            "add-button open"
        };
        let button_content = if adding { "-" } else { "+" };
        let add_section_class = if adding {
            "add-section"
        } else {
            "add-section hidden"
        };
        html! {
            <section class="new-entry">
                <section class="add-toggle">
                    <button class=button_class
                        onclick=|_| { if adding { BlogMsg::HideAdd } else { BlogMsg::ShowAdd } }
                    >
                        { button_content }
                    </button>
                </section>
                <section class=add_section_class>
                    <div>
                        <div>
                            <input class="new-entry-title title"
                                placeholder="Name your thoughts"
                                value=&self.state.inflight.title
                                oninput=|e| BlogMsg::UpdateTitle(e.value)
                            />
                        </div>
                        <div>
                            <textarea class="new-entry-body body"
                                placeholder="Share your thoughts"
                                value=&self.state.inflight.body
                                oninput=|e| BlogMsg::UpdateBody(e.value)
                            />
                        </div>
                        <div>
                            <button class="add-blog-entry" onclick=|_| BlogMsg::Add>
                                { format!("Save Entry") }
                            </button>
                        </div>
                    </div>
                </section>
            </section>
        }
    }

    fn view_entry(entry: &BlogEntry) -> Html<Blog> {
        html! {
            <li class="entry">
                <span class="time" title=entry.time.with_timezone(&Local) >
                    { format!("{}", entry.time.with_timezone(&Local).format("%v")) }
                </span>
                <div class="header">
                    <span class="title">
                        { format!("{}", entry.title) }
                    </span>
                </div>
                <div class="body">
                    { format!("{}", entry.body) }
                </div>
            </li>
        }
    }
}

pub enum BlogMsg {
    Add,
    ShowAdd,
    HideAdd,
    UpdateTitle(String),
    UpdateBody(String),
    // GetPage(usize),
}

impl BlogState {
    fn _get_page(&self, page: usize) -> Option<&[BlogEntry]> {
        let len = self.entries.len();

        let start = page * self.page_size;
        let end = min(start + self.page_size, len);

        if start < len && end <= len {
            return self.entries.get(start..end);
        } else {
            None
        }
    }

    fn next_id(&self) -> usize {
        let max_o = self.entries.iter().max_by_key(|e| e.id);
        if let Some(max) = max_o {
            return max.id + 1;
        }
        1
    }
}

impl Component for Blog {
    type Message = BlogMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);
        let entries = {
            if let Json(Ok(restored_blog)) = storage.restore(BLOG_KEY) {
                restored_blog
            } else {
                Vec::new()
            }
        };
        let state = BlogState {
            inflight: BlogEntry::new(),
            entries,
            adding: false,
            page_size: PAGE_SIZE,
        };
        Blog { storage, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BlogMsg::Add => {
                let mut entry = self.state.inflight.clone();
                entry.id = self.state.next_id();
                self.state.inflight = BlogEntry::new();
                self.state.entries.push(entry);
            }
            BlogMsg::ShowAdd => {
                self.state.adding = true;
            }
            BlogMsg::HideAdd => {
                self.state.adding = false;
                self.state.inflight = BlogEntry::new();
            }
            BlogMsg::UpdateTitle(title) => {
                self.state.inflight.title = title;
            }
            BlogMsg::UpdateBody(body) => {
                self.state.inflight.body = body;
            } // BlogMsg::GetPage(_page) => {}
        }
        self.storage.store(BLOG_KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="blog-wrapper">
                <section class="blogapp">
                    <header class="header">
                        <h1>{ BLOG_TITLE }</h1>
                        { self.view_input() }
                    </header>
                </section>
                <section class="previous-thoughts">
                    <ul class="entries">
                        { for self.state.entries.iter().rev().map(Self::view_entry) }
                    </ul>
                </section>
            </div>
        }
    }
}
