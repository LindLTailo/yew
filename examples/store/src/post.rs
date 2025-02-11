use crate::agents::posts::{PostId, PostRequest, PostStore};
use crate::text_input::TextInput;
use yew::prelude::*;
use yew::utils::NeqAssign;
use yew_agent::utils::store::{Bridgeable, ReadOnly, StoreWrapper};
use yew_agent::Bridge;

pub enum Msg {
    UpdateText(String),
    Delete,
    PostStore(ReadOnly<PostStore>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: PostId,
}

pub struct Post {
    link: ComponentLink<Self>,
    id: PostId,
    text: Option<String>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::PostStore);
        Self {
            link,
            id: props.id,
            text: None,
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(text) => {
                self.post_store.send(PostRequest::Update(self.id, text));
                false
            }
            Msg::Delete => {
                self.post_store.send(PostRequest::Remove(self.id));
                false
            }
            Msg::PostStore(state) => {
                let state = state.borrow();

                // Only update if the post changed.
                if let Some(text) = state.posts.get(&self.id) {
                    self.text.neq_assign(Some(text.clone()))
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id.neq_assign(props.id)
    }

    fn view(&self) -> Html {
        let text = self.text.as_deref().unwrap_or("<pending>");

        html! {
            <div>
                <h2>{ format!("Post #{}", self.id) }</h2>
                <p>{text}</p>

                <TextInput value={text.to_owned()} onsubmit={self.link.callback(Msg::UpdateText)} />
                <button onclick={self.link.callback(|_| Msg::Delete)}>
                    { "Delete" }
                </button>
            </div>
        }
    }
}
