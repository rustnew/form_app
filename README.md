Voici un guide détaillé pour configurer, exécuter et déployer votre projet fullstack Rust utilisant Yew pour le frontend et un backend API REST (par exemple avec Axum ou Actix Web).

---

## 🛠️ Prérequis

Avant de commencer, assurez-vous d'avoir installé les outils suivants :

- **Rust** :Installez Rust via [rustup](https://rustup.rs/)
- **Trunk** :Outil pour compiler et servir des applications Yew. Installez-le avec 
 
```bash
  cargo install --locked trunk
  ```
- **Cible WebAssembly** :Ajoutez la cible de compilation WebAssembly 
 
```bash
  rustup target add wasm32-unknown-unknown
  ```

---

## 📁 Structure du Projet

Organisez votre projet comme suit :

```plaintext
mon-projet/
├── frontend/          # Application Yew
│   ├── src/
│   │   └── main.rs
│   ├── index.html
│   └── Cargo.toml
├── backend/           # API REST en Rust
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── Cargo.toml         # Fichier racine (optionnel)
└── README.md
``


---

## 🚀 Lancement du Frontend (Yew)

1. **Naviguez dans le dossier frontend** :
   ```bash
   cd frontend
   ``


2. **Lancez l'application en mode développement** :
   ```bash
   trunk serve
   ``

   Cela compile votre application en WebAssembly et la sert localement. Par défaut, elle est accessible à l'adresse [http://localhost:8080](http://localhost:8080).

3. **Compilation en mode production** :
   ```bash
   trunk build --release
   ``

   Les fichiers compilés seront placés dans le dossier `dist/`.

---

## 🧪 Lancement du Backend (API REST)

1. **Naviguez dans le dossier backend** :
   ```bash
   cd backend
   ``


2. **Lancez le serveur backend** :
   ```bash
   cargo run
   ``

   Assurez-vous que votre serveur écoute sur `http://localhost:8080/api/submit` pour correspondre aux requêtes du frontend.

---

## 🌐 Déploiement

### Frontend (Yew)

1. **Compilation en mode production** :
   ```bash
   trunk build --release
  ```


2. **Configuration du serveur** :
   Configurez votre serveur HTTP pour servir le fichier `index.html` pour toutes les routes, surtout si vous utilisez le routeur de Yew. Cela garantit que les routes côté client fonctionnent correctement.

   Par exemple, avec Nginx :

   ```nginx
   location / {
       try_files $uri /index.html;
   }
  ```


### Backnd

Déployez votre backend comme toute autre application Rust, en utilisant des outils comme Docker, ou en le déployant sur un serveur coud.

---

## 📚 Ressources Supplémentaires

- **Documentation officielle de Ye** : [https://yew.rs/docs/tutorial](https://yew.rs/docs/tutrial)
- **Guide de déploiement de Ye** : [https://yew.rs/docs/more/deployment](https://yew.rs/docs/more/deploment)
- **Exemple de projet fullstack avec Yew et Axu** : [https://implfuture.dev/blog/rewriting-the-modern-web-in-rust](https://implfuture.dev/blog/rewriting-the-modern-web-inrust)

---

Souhaitez-vous que je vous fournisse un exemple de configuration pour le backend avec Axum ou Actix Web, ou avez-vous besoin d'aide pour un aspect spécifique du projet ? 
