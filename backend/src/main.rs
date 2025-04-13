use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder};
use log::info;
use actix_cors::Cors;
use actix_web::{ App, HttpServer};



#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub nom: String,
    pub prenom: String,
    pub numero: String,
}

// Réponse pour confirmer l'enregistrement réussi
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}


pub async fn submit_form(person: web::Json<Person>) -> impl Responder {
    // Dans un projet réel, vous pourriez sauvegarder dans une base de données ici
    info!(
        "Personne reçue: {} {}, numéro: {}",
        person.prenom, person.nom, person.numero
    );

    // Renvoyer une réponse de succès
    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        message: format!(
            "Formulaire soumis avec succès pour {} {}",
            person.prenom, person.nom
        ),
    })
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Service opérationnel")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser le logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Démarrage du serveur sur http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        // Configuration CORS pour permettre les requêtes depuis le frontend
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .route("/health", web::get().to(health_check))
            .route("/api/submit", web::post().to(submit_form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}