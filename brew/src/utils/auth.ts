import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { jwtDecode } from "jwt-decode";
import { z } from "zod";
import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

export function getGoogleOAuthLink() {
  return `https://accounts.google.com/o/oauth2/v2/auth?client_id=${
    import.meta.env.VITE_GOOGLE_OAUTH_CLIENT_ID
  }&redirect_uri=${
    import.meta.env.VITE_GOOGLE_OAUTH_REDIRECT_URI
  }&response_type=token&scope=email profile`;
}

type UserStore = {
  info?: {
    email: string;
    givenName: string;
    familyName: string;
    picture: string;
    id: number;
    tok: string;
  };

  setWithToken: (jwt: string) => void;
};

const authInfoSchema = z.object({
  email: z.string(),
  given_name: z.string(),
  family_name: z.string(),
  picture: z.string(),
  id: z.number(),
});

export const useUserStore = create<UserStore>()(
  persist(
    (set) => ({
      setWithToken(jwt) {
        const res = authInfoSchema.parse(jwtDecode(jwt));

        set({
          info: {
            email: res.email,
            familyName: res.family_name,
            givenName: res.given_name,
            id: res.id,
            picture: res.picture,
            tok: jwt,
          },
        });
      },
    }),
    {
      name: "dev.holewinski.kaufy.user",
      storage: createJSONStorage(() => sessionStorage),
    },
  ),
);

export function useAuth(): void {
  const nav = useNavigate();
  const info = useUserStore((s) => s.info);

  useEffect(() => {
    if (!info) nav("/login");
  }, [info]);
}
