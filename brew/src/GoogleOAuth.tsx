import { useLocation, useNavigate } from "react-router-dom";
import { z } from "zod";
import { useEffect } from "react";
import { useUserStore } from "@/utils/auth";

const googleAccessTokenInfoSchema = z.object({ azp: z.string() });

const GoogleOAuth = () => {
  const { hash } = useLocation();
  const nav = useNavigate();
  const token = hash.slice("#access_token=".length).split("&")[0];
  const { setWithToken } = useUserStore.getState();

  async function processToken(access_token: string) {
    const res = await fetch(
      `https://www.googleapis.com/oauth2/v3/tokeninfo?access_token=${access_token}`,
    )
      .then((res) => res.json())
      .then(googleAccessTokenInfoSchema.parse);
    if (res.azp != import.meta.env.VITE_GOOGLE_OAUTH_CLIENT_ID)
      throw new Error("Invalid authorized presenter");

    const { token } = await fetch(
      import.meta.env.VITE_API_BASE_URL + "/oauth/google?token=" + access_token,
      {
        method: "POST",
      },
    ).then((res) => res.json());

    setWithToken(token);
    nav("/");
  }

  useEffect(() => {
    processToken(token);
  }, [token]);

  return <pre>loading...</pre>;
};

export default GoogleOAuth;
