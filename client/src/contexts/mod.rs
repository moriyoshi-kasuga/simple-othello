use std::rc::Rc;

use yew::prelude::*;

use crate::services::connection::Connection;

#[derive(Clone)]
pub struct AppContext {
    pub connection: Rc<Connection>,
}

impl PartialEq for AppContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.connection, &other.connection)
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct AppContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn AppContextProvider(props: &AppContextProviderProps) -> Html {
    let websocket = use_memo(0u8, |_| Connection::new());

    let context = AppContext {
        connection: websocket.clone(),
    };

    html! {
        <ContextProvider<Rc<AppContext>> context={Rc::new(context)}>
            { props.children.clone() }
        </ContextProvider<Rc<AppContext>>>
    }
}

pub fn use_app_context() -> impl Hook<Output = Rc<AppContext>> {
    struct AppContextHook;

    impl Hook for AppContextHook {
        type Output = Rc<AppContext>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            use_context::<Rc<AppContext>>().run(ctx).expect(
                "No AppContext found. Make sure your component is wrapped in <AppContextProvider>",
            )
        }
    }

    AppContextHook
}
