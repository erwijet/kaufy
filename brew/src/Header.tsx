import { useState } from "react";
import {
  Container,
  Group,
  Burger,
  Avatar,
  Text,
  Menu,
  UnstyledButton,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useAuth, useUserStore } from "./utils/auth";
import { LuChevronDown, LuLogOut } from "react-icons/lu";
import { IconContext } from "react-icons";

const links = [
  { link: "/drinks", label: "Drinks" },
  { link: "/orders", label: "Orders" },
];

export const Header = () => {
  useAuth();

  const [opened, { toggle }] = useDisclosure(false);
  const [active, setActive] = useState(links[0].link);
  const [isUserMenuOpen, setUserMenuOpen] = useState(false);

  const userInfo = useUserStore((s) => s.info);

  const items = links.map((link) => (
    <a
      key={link.label}
      href={link.link}
      className="block leading-[1] py-2 px-3 rounded-sm no-underline text-white"
      data-active={active === link.link || undefined}
      onClick={(event) => {
        event.preventDefault();
        setActive(link.link);
      }}
    >
      {link.label}
    </a>
  ));

  return (
    <header className="h-14 mb-32 bg-gray-900">
      <Container size="md" className="h-14 flex justify-between items-center">
        <h5 className={"font-extrabold text-2xl"}>
          <Text
            component="span"
            variant="gradient"
            gradient={{ from: "blue", to: "cyan" }}
            inherit
          >
            Kaufy
          </Text>
        </h5>
        <Group gap={7} visibleFrom="xs">
          {items}

          <Menu
            width={260}
            position="bottom-end"
            transitionProps={{ transition: "pop-bottom-right" }}
            onOpen={() => setUserMenuOpen(true)}
            onClose={() => setUserMenuOpen(false)}
          >
            <Menu.Target>
              <UnstyledButton>
                <Group ml={"lg"}>
                  <Avatar
                    src={userInfo?.picture}
                    alt={userInfo?.email}
                    radius="xl"
                    size={20}
                  />
                  <Text fw={500} size="sm" lh={1} mr={3} className="text-white">
                    {userInfo?.givenName} {userInfo?.familyName}
                  </Text>
                  <IconContext.Provider
                    value={{
                      color: "white",
                      className: `transition-transform ${
                        isUserMenuOpen ? "rotate-180" : "rotate-0"
                      }`,
                    }}
                  >
                    <LuChevronDown />
                  </IconContext.Provider>
                  );
                </Group>
              </UnstyledButton>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Item leftSection={<LuLogOut />}>Log Out</Menu.Item>
            </Menu.Dropdown>
          </Menu>
        </Group>

        <Burger opened={opened} onClick={toggle} hiddenFrom="xs" size="sm" />
      </Container>
    </header>
  );
};
