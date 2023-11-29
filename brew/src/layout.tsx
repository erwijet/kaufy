import { useEffect, useReducer, useRef } from "react";
import { Navbar } from "@/Navbar";
import { Outlet } from "react-router-dom";

export const Layout = () => {
  const ref = useRef<HTMLElement>(null);
  const [, forceUpdate] = useReducer((x) => x + 1, 0);

  // redraw the body once we have computed the navbar width
  useEffect(() => void forceUpdate(), [ref.current]);

  return (
    <div className="w-screen h-fit">
      <Navbar ref={ref} />
      <div
        style={{
          paddingLeft: ref.current?.getBoundingClientRect().width,
        }}
      >
        <Outlet />
      </div>
    </div>
  );
};
