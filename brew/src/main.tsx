import {
  ApolloClient,
  ApolloProvider,
  InMemoryCache,
  createHttpLink,
} from "@apollo/client";
import { MantineProvider, Notification, createTheme } from "@mantine/core";
import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes, useNavigate } from "react-router-dom";

import { setContext } from "@apollo/client/link/context";

import "@/main.css";
import "@mantine/core/styles.css";

import Drinks from "@/Drinks";
import GoogleOAuth from "@/GoogleOAuth";
import Home from "@/Home";
import Login from "@/Login";
import Logout from "@/Logout";
import Orders from "@/Orders";
import Settings from "@/Settings";
import { useUserStore } from "@/utils/auth";
import toast, { ToastBar, Toaster } from "react-hot-toast";
import { Layout } from "./layout";
import { Notifier } from "./notifier";

const theme = createTheme({
  fontFamily: "Geist",
});

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
    <MantineProvider theme={theme} defaultColorScheme="dark">
      <ApolloProvider client={gqlClient}>
        <Toaster position="bottom-center">
          {(t) => (
            <ToastBar toast={t} style={{ padding: "0px" }}>
              {({ icon, message }) => (
                <Notification
                  title={icon}
                  withBorder
                  withCloseButton
                  onClose={() => toast.dismiss(t.id)}
                >
                  {message}
                </Notification>
              )}
            </ToastBar>
          )}
        </Toaster>
        <Notifier />
        <BrowserRouter>
          <Routes>
            <Route path="login" Component={Login} />
            <Route element={<Layout />}>
              <Route
                index
                Component={() => {
                  const nav = useNavigate();
                  useEffect(() => nav("/home"));
                  return <></>;
                }}
              />
              <Route path="logout" Component={Logout} />
              <Route path="home" Component={Home} />
              <Route path="orders" Component={Orders} />
              <Route path="drinks" Component={Drinks} />
              <Route path="settings" Component={Settings} />
              <Route path="oauth/google" Component={GoogleOAuth} />
            </Route>
          </Routes>
        </BrowserRouter>
      </ApolloProvider>
    </MantineProvider>
  </React.StrictMode>,
);
