import { Container, Text, Button, Group } from "@mantine/core";
import { FaGoogle, FaGithub } from "react-icons/fa";

import { getGoogleOAuthLink } from "@/utils/auth";

const Login = () => {
  return (
    <div className={"relative box-border bg-gray-800"}>
      <Container
        size={700}
        className={
          "relative flex h-screen items-center justify-center flex-col"
        }
      >
        <h1 className={"font-extrabold text-white text-6xl"}>
          Welcome to{" "}
          <Text
            component="span"
            variant="gradient"
            gradient={{ from: "blue", to: "cyan" }}
            inherit
          >
            Kaufy
          </Text>
        </h1>

        <Text mt={"xl"} c="dimmed">
          Build fully functional accessible web applications with ease - Mantine
          includes more than 100 customizable components and hooks to cover you
          in any situation
        </Text>

        <Group mt={"xl"}>
          <Button
            component="a"
            href={getGoogleOAuthLink()}
            size="xl"
            variant="gradient"
            className="h-[54px] px-[18px] flex-1"
            gradient={{ from: "blue", to: "cyan" }}
            leftSection={<FaGoogle />}
          >
            Continue with Google
          </Button>

          <Button
            component="a"
            href="https://github.com/erwijet/kaufy"
            size="xl"
            variant="default"
            className="h-[54px] px-[18px] flex-1"
            leftSection={<FaGithub />}
          >
            View on Github
          </Button>
        </Group>
      </Container>
    </div>
  );
};

export default Login;
