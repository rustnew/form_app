use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Person {
    nom: String,
    prenom: String,
    numero: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct SuccessResponse {
    success: bool,
    message: String,
}

#[derive(Clone, PartialEq)]
enum NotificationStatus {
    None,
    Success(String),
    Error(String),
}

#[function_component(PersonForm)]
pub fn person_form() -> Html {
    
    let nom = use_state(|| String::new());
    let prenom = use_state(|| String::new());
    let numero = use_state(|| String::new());
    let notification = use_state(|| NotificationStatus::None);
    let is_submitting = use_state(|| false);

    let on_submit = {
        let nom = nom.clone();
        let prenom = prenom.clone();
        let numero = numero.clone();
        let notification = notification.clone();
        let is_submitting = is_submitting.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let nom_val = (*nom).clone();
            let prenom_val = (*prenom).clone();
            let numero_val = (*numero).clone();
            let notification = notification.clone();
            let is_submitting: UseStateHandle<bool> = is_submitting.clone();
            
            // Validation simple
            if nom_val.trim().is_empty() || prenom_val.trim().is_empty() || numero_val.trim().is_empty() {
                notification.set(NotificationStatus::Error("Tous les champs sont obligatoires".to_string()));
                return;
            }
            
            // Création de la personne à envoyer
            let person = Person {
                nom: nom_val,
                prenom: prenom_val,
                numero: numero_val,
            };
            
            // Marquer comme en cours de soumission
            is_submitting.set(true);
            
            // Utiliser wasm_bindgen_futures pour gérer l'appel asynchrone
            wasm_bindgen_futures::spawn_local(async move {
                match submit_person(&person).await {
                    Ok(response) => {
                        notification.set(NotificationStatus::Success(response.message));
                    }
                    Err(err) => {
                        notification.set(NotificationStatus::Error(format!("Erreur: {}", err)));
                    }
                }
                is_submitting.set(false);
            });
        })
    };

    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            state.set(input.value());
        })
    };

    html! {
        <div class="form-container">
            <h1>{"Formulaire d'inscription"}</h1>
            
            {
                match (*notification).clone() {
                    NotificationStatus::Success(msg) => html! {
                        <div class="notification success">{ msg }</div>
                    },
                    NotificationStatus::Error(msg) => html! {
                        <div class="notification error">{ msg }</div>
                    },
                    NotificationStatus::None => html! {}
                }
            }
            
            <form onsubmit={on_submit}>
                <div class="form-group">
                    <label for="prenom">{"Prénom:"}</label>
                    <input 
                        type="text" 
                        id="prenom" 
                        value={(*prenom).clone()} 
                        onchange={handle_input(prenom.clone())}
                    />
                </div>
                
                <div class="form-group">
                    <label for="nom">{"Nom:"}</label>
                    <input 
                        type="text" 
                        id="nom" 
                        value={(*nom).clone()} 
                        onchange={handle_input(nom.clone())}
                    />
                </div>
                
                <div class="form-group">
                    <label for="numero">{"Numéro de téléphone:"}</label>
                    <input 
                        type="tel" 
                        id="numero" 
                        value={(*numero).clone()} 
                        onchange={handle_input(numero.clone())}
                    />
                </div>
                
                <button type="submit" disabled={*is_submitting}>
                    { if *is_submitting { "Envoi en cours..." } else { "Envoyer" } }
                </button>
            </form>
        </div>
    }
}

async fn submit_person(person: &Person) -> Result<SuccessResponse, String> {
    let response = Request::post("http://localhost:8080/api/submit")
        .header("Content-Type", "application/json")
        .json(person)
        .map_err(|e| format!("Erreur de sérialisation: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Erreur réseau: {}", e))?;
    
    if response.ok() {
        let data = response
            .json::<SuccessResponse>()
            .await
            .map_err(|e| format!("Erreur de désérialisation: {}", e))?;
        Ok(data)
    } else {
        Err(format!("Erreur serveur: {}", response.status()))
    }
}



#[function_component(App)]
fn app() -> Html {
    html! {
        <PersonForm />
    }
}

fn main() {
    // Initialiser le logger pour le débogage
    wasm_logger::init(wasm_logger::Config::default());
    
    // Monter l'application Yew
    yew::Renderer::<App>::new().render();
}
