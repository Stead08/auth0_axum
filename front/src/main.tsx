import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import {Auth0Provider} from "@auth0/auth0-react";


ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <Auth0Provider
          domain={import.meta.env.VITE_AUTH0_DOMAIN}
          clientId={import.meta.env.VITE_AUTH0_CLIENT_ID}
          authorizationParams={{
            redirect_uri: window.location.origin,
            audience: `${import.meta.env.VITE_AUTH0_AUDIENCE}`,
            scope: "read:current_user update:current_user_metadata"
          }}
      >
        <App />
      </Auth0Provider>
    </React.StrictMode>
)
