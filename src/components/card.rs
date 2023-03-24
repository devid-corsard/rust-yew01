use yew::prelude::*;
use crate::models::user::User;

#[derive(Properties, PartialEq)]
pub struct CardProp {
    pub user: User,
}

#[function_component]
pub fn Card(CardProp { user }: &CardProp) -> Html {
    html! {
    <div class="m-3 p-4 border rounded d-flex align-items-center">
        <img src={format! ("{}", user.image.clone())} class="img-thumbnail mr-2 w-25 p-3" alt="img" />
        <div class="m-4">
            <p class="fw-bold mb-1">{format!("{} {}", user.first_name.clone(), user.last_name.clone())}</p>
            <p class="fw-normal mb-1">{user.gender.clone()}</p>
            <p class="fw-normal mb-1">{user.email.clone()}</p>
            <p class="fw-normal mb-1">{user.phone.clone()}</p>
        </div>
    </div>
    }
}
