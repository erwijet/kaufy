import { useMutation, useQuery } from "@apollo/client";
import {
  ActionIcon,
  Avatar,
  Badge,
  Burger,
  Button,
  Container,
  Group,
  Modal,
  Table,
  Text,
  TextInput,
  Title,
  rem,
} from "@mantine/core";
import { useState } from "react";
import { LuPencil, LuTrash2 } from "react-icons/lu";
import { DrinkBuilder } from "./DrinkBuilder";
import { graphql } from "./gql";
import { useAuth, useUserStore } from "./utils/auth";
import { getGradientFromNumber } from "./utils/color";

const drinksQuery = graphql(`
  query drinks {
    drinks {
      id
      name
      temperature {
        name
      }
      base {
        name
      }
      owner {
        email
        givenName
        familyName
        picture
      }
      addons {
        id
        name
      }
    }
  }
`);

const deleteDrinkMutation = graphql(`
  mutation DeleteDrinkById($id: Int!) {
    deleteDrink(id: $id) {
      success
    }
  }
`);

const Drinks = () => {
  useAuth();

  const email = useUserStore((s) => s.info?.email ?? "");
  const { data, refetch } = useQuery(drinksQuery);
  const [search, setSearch] = useState("");
  const [activeId, setActiveId] = useState(null as null | "new" | number);

  const [deleteDrink] = useMutation(deleteDrinkMutation, {
    onCompleted: () => refetch(),
  });

  const [navOpen, setNavOpen] = useState(false);

  return (
    <>
      <Modal
        centered
        opened={activeId != null}
        onClose={() => setActiveId(null)}
        title={activeId == "new" ? "Create Drink" : "Edit Drink"}
      >
        <DrinkBuilder
          id={activeId ?? "new"}
          onDone={() => {
            setActiveId(null);
            refetch();
          }}
        />
      </Modal>
      <div>
        <div>
          <Burger
            m={"md"}
            opened={navOpen}
            hiddenFrom="sm"
            size={"sm"}
            onClick={() => setNavOpen(!navOpen)}
          />
        </div>
        <div>
          <Container size={"xl"}>
            <div className="flex justify-between items-center my-8">
              <Title>Drinks</Title>
              <div className="flex items-center gap-1">
                <TextInput
                  variant="filled"
                  placeholder="Search by Name..."
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                />
                <Button onClick={() => setActiveId("new")}>New</Button>
              </div>
            </div>
            <Table>
              <Table.Thead>
                <Table.Tr>
                  <Table.Th>Drink Name</Table.Th>
                  <Table.Th>Base</Table.Th>
                  <Table.Th>Addons</Table.Th>
                  <Table.Th>Owner</Table.Th>
                </Table.Tr>
              </Table.Thead>
              <Table.Tbody>
                {data?.drinks
                  .filter(
                    (drink) =>
                      !search ||
                      drink.name.toLowerCase().includes(search.toLowerCase()),
                  )
                  .map((drink) => (
                    <Table.Tr>
                      <Table.Td className="items-center h-full">
                        <Group gap="xs" wrap="nowrap">
                          <Text size="sm">{drink.name}</Text>
                          <Badge
                            variant="outline"
                            className="ml-2"
                            color={
                              drink.temperature.name.toUpperCase() == "ICED"
                                ? "cyan"
                                : "red"
                            }
                          >
                            {drink.temperature.name}
                          </Badge>
                        </Group>
                      </Table.Td>
                      <Table.Td>
                        <Text size="sm">{drink.base.name}</Text>
                      </Table.Td>
                      <Table.Td>
                        <Group gap={"xs"} maw={rem(300)}>
                          {drink.addons.map((addon) => (
                            <Badge
                              size="sm"
                              variant="gradient"
                              gradient={getGradientFromNumber(addon.id)}
                            >
                              {addon.name}
                            </Badge>
                          ))}
                        </Group>
                      </Table.Td>
                      <Table.Td className="flex gap-2 items-center">
                        <Group gap={"xs"}>
                          <Avatar
                            src={drink.owner.picture}
                            size={"xs"}
                            imageProps={{ referrerPolicy: "no-referrer" }}
                          />
                          {drink.owner.email == email ? (
                            <Badge variant="default">Me</Badge>
                          ) : (
                            <Text size="sm">
                              {drink.owner.givenName} {drink.owner.familyName}
                            </Text>
                          )}
                        </Group>
                      </Table.Td>
                      <Table.Td>
                        {drink.owner.email == email ? (
                          <Group justify="center" wrap="nowrap" gap={"xs"}>
                            <ActionIcon
                              variant="transparent"
                              color="gray"
                              size="sm"
                              aria-label="actions"
                              onClick={() => setActiveId(drink.id)}
                            >
                              <LuPencil />
                            </ActionIcon>
                            <ActionIcon
                              variant="transparent"
                              color="gray"
                              size="sm"
                              aria-label="actions"
                              onClick={() =>
                                deleteDrink({ variables: { id: drink.id } })
                              }
                            >
                              <LuTrash2 />
                            </ActionIcon>
                          </Group>
                        ) : null}
                      </Table.Td>
                    </Table.Tr>
                  ))}
              </Table.Tbody>
            </Table>
          </Container>
        </div>
      </div>
    </>
  );
};

export default Drinks;
