import { useDefer } from '@tsly/hooks';
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useUserStore } from "@/utils/auth";

const Logout = () => {
  const clear = useUserStore.getState().clear;
  const nav = useNavigate();

  const deferNav = useDefer((to: string) => nav(to));

  useEffect(() => {
    clear();
    deferNav('/');
  }, []);

  return <>Loading...</>;
};

export default Logout;
