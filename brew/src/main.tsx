import React from "react";
import ReactDOM from "react-dom/client";
import { MantineProvider, createTheme } from "@mantine/core";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import {
  ApolloClient,
  InMemoryCache,
  ApolloProvider,
  createHttpLink,
} from "@apollo/client";

import { setContext } from "@apollo/client/link/context";

import "@mantine/core/styles.css";
import "@/main.css";

import App from "@/App";
import Login from "@/Login";
import GoogleOAuth from "@/GoogleOAuth";
import { useUserStore } from "@/utils/auth";

const theme = createTheme({});

const httpLink = createHttpLink({
  uri: import.meta.env.VITE_API_BASE_URL + "/graphql",
});

const authLink = setContext((_, { headers }) => {
  const tok = useUserStore.getState().info?.tok;

  return {
    headers: {
      ...headers,
      authorization: tok ? `Bearer ${tok}` : "",
    },
  };
});

const gqlClient = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache(),
});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <MantineProvider theme={theme}>
      <ApolloProvider client={gqlClient}>
        <BrowserRouter>
          <Routes>
            <Route index Component={App} />
            <Route path="login" Component={Login} />
            <Route path="oauth/google" Component={GoogleOAuth} />
          </Routes>
        </BrowserRouter>
      </ApolloProvider>
    </MantineProvider>
  </React.StrictMode>,
);
