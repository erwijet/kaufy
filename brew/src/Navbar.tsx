import { UnstyledButton, Tooltip, rem } from "@mantine/core";
import {
  LuHome,
  LuCoffee,
  LuClipboard,
  LuLogOut,
  LuSettings,
  LuCode,
} from "react-icons/lu";
import classes from "@/Navbar.module.css";
import { IconContext } from "react-icons";
import { useLocation, useNavigate } from "react-router-dom";
import { useUserStore } from "@/utils/auth";
import { forwardRef } from "react";

const topLinksData = [
  { icon: LuHome, label: "Home", path: "/home" },
  { icon: LuCoffee, label: "Drinks", path: "/drinks" },
  { icon: LuClipboard, label: "Orders", path: "/orders" },
];

const bottomLinksData = [
  { icon: LuSettings, label: "Settings", path: "/settings" },
  { icon: LuLogOut, label: "Logout", path: "/logout" },
];

export const Navbar = forwardRef<HTMLElement>((_props, ref) => {
  const location = useLocation();
  const nav = useNavigate();

  const topLinks = topLinksData.map((link) => (
    <Tooltip
      label={link.label}
      position="right"
      withArrow
      transitionProps={{ duration: 0 }}
      key={link.label}
    >
      <UnstyledButton
        onClick={() => nav(link.path)}
        className={classes.mainLink}
        data-active={link.path == location.pathname || undefined}
      >
        <IconContext.Provider
          value={{
            style: { width: rem(22), height: rem(22), stroke: "1.5" },
          }}
        >
          <link.icon />
        </IconContext.Provider>
      </UnstyledButton>
    </Tooltip>
  ));

  const bottomLinks = bottomLinksData.map((link) => (
    <Tooltip
      label={link.label}
      position="right"
      withArrow
      transitionProps={{ duration: 0 }}
      key={link.label}
    >
      <UnstyledButton
        onClick={() => nav(link.path)}
        className={classes.mainLink}
        data-active={link.path == location.pathname || undefined}
      >
        <IconContext.Provider
          value={{
            style: { width: rem(22), height: rem(22), stroke: "1.5" },
          }}
        >
          <link.icon />
        </IconContext.Provider>
      </UnstyledButton>
    </Tooltip>
  ));

  return (
    <nav className={classes.navbar} ref={ref}>
      <div className={classes.wrapper}>
        <div className={classes.aside}>
          <div className="flex flex-col gap-1">{topLinks}</div>
          <div className="flex flex-col gap-1">
            <Tooltip
              label={"GQL Playground"}
              position="right"
              withArrow
              transitionProps={{ duration: 0 }}
              key={"GQL Playground"}
            >
              <UnstyledButton
                onClick={() =>
                  (window.location.href = `${
                    import.meta.env.VITE_API_BASE_URL
                  }/graphql?tok=${useUserStore.getState().info?.tok ?? ""}`)
                }
                className={classes.mainLink}
              >
                <IconContext.Provider
                  value={{
                    style: { width: rem(22), height: rem(22), stroke: "1.5" },
                  }}
                >
                  <LuCode />
                </IconContext.Provider>
              </UnstyledButton>
            </Tooltip>
            {bottomLinks}
          </div>
        </div>
      </div>
    </nav>
  );
});
