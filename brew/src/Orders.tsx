import { useMutation, useQuery } from "@apollo/client";
import {
  ActionIcon,
  Anchor,
  Avatar,
  Badge,
  Button,
  Container,
  Group,
  Loader,
  Modal,
  MultiSelect,
  Select,
  Table,
  Text,
  TextInput,
  Title,
  rem,
} from "@mantine/core";
import { maybeParseInt } from "@tsly/core";
import { maybe } from "@tsly/maybe";
import { useEffect, useRef, useState } from "react";
import { LuTrash2 } from "react-icons/lu";
import { graphql } from "./gql";
import { useAuth, useUserStore } from "./utils/auth";

const ordersQuery = graphql(`
  query orders {
    orders {
      id
      drink {
        id
        name
      }
      requester {
        picture
        email
        givenName
        familyName
      }
      status
      requestedAt
      requestedFor
    }
  }
`);

const NewOrderModal = (props: { opened: boolean; onClose: () => void }) => {
  const ref = useRef<HTMLInputElement>(null);
  const { data } = useQuery(
    graphql(`
      query drinkNamesAndIds {
        drinks {
          id
          name
        }
      }
    `),
  );

  const [addOrder, { loading }] = useMutation(
    graphql(`
      mutation AddOrder($drinkId: Int!) {
        addOrder(input: { drinkId: $drinkId }) {
          status
        }
      }
    `),
    {
      onCompleted: () => props.onClose(),
    },
  );

  useEffect(() => {
    const timeout = setTimeout(() => ref.current?.focus(), 5);
    return () => clearTimeout(timeout);
  }, [props.opened]);

  return (
    <Modal
      centered
      opened={props.opened}
      onClose={() => props.onClose()}
      title={"Place Order"}
    >
      {loading ? (
        <Loader display={"block"} m={"auto"} mb={"md"} />
      ) : (
        <Select
          ref={ref}
          placeholder="Drink..."
          data={
            data?.drinks.map((each) => ({
              value: each.id.toString(),
              label: each.name,
            })) ?? []
          }
          onChange={(e) =>
            maybe(e)
              ?.let(maybeParseInt)
              ?.take((drinkId) => addOrder({ variables: { drinkId } }))
          }
          searchable
        />
      )}
    </Modal>
  );
};

const Orders = () => {
  useAuth();

  const email = useUserStore((s) => s.info?.email ?? "");
  const { data, refetch } = useQuery(ordersQuery, { fetchPolicy: "no-cache" });

  const [search, setSearch] = useState("");
  const [filterStatues, setFilterStatuses] = useState(["Queued", "Open"]);
  const [isNewOrderModalOpen, setNewOrderModalOpen] = useState(false);

  const [cancelOrder, {}] = useMutation(
    graphql(`
      mutation CancelOrder($id: Int!) {
        cancelOrder(id: $id) {
          status
        }
      }
    `),
    {
      onCompleted: () => refetch(),
    },
  );

  return (
    <>
      <NewOrderModal
        opened={isNewOrderModalOpen}
        onClose={() => {
          setNewOrderModalOpen(false);
          refetch();
        }}
      />
      <Container size={"xl"}>
        <div className="flex justify-between items-center my-8">
          <div className="flex items-center gap-4">
            <Title>Orders</Title>
            <MultiSelect
              w={rem(256)}
              data={["Open", "Queued", "Done", "Cancelled"]}
              value={filterStatues}
              onChange={(statuses) => setFilterStatuses(statuses)}
            />
          </div>
          <div className="flex items-center gap-1">
            <TextInput
              variant="filled"
              placeholder="Search..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
            <Button onClick={() => setNewOrderModalOpen(true)}>Order</Button>
          </div>
        </div>
        <Table>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Order No.</Table.Th>
              <Table.Th>Requester</Table.Th>
              <Table.Th>Drink</Table.Th>
              <Table.Th>Requested For</Table.Th>
              <Table.Th>Status</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {data?.orders
              .filter(
                (order) =>
                  !search ||
                  order.drink.name.toLowerCase().includes(search.toLowerCase()),
              )
              .filter((order) =>
                filterStatues
                  .map((each) => each.toUpperCase())
                  .includes(order.status),
              )
              .map((order) => (
                <Table.Tr>
                  <Table.Td className="items-center h-full">
                    <Text size="sm">{`Order #${order.id}`}</Text>
                  </Table.Td>
                  <Table.Td className="flex gap-2 items-center">
                    <Group gap={"xs"}>
                      <Avatar
                        src={order.requester.picture}
                        size={"xs"}
                        imageProps={{ referrerPolicy: "no-referrer" }}
                      />
                      {order.requester.email == email ? (
                        <Badge variant="default">Me</Badge>
                      ) : (
                        <Text size="sm">
                          {order.requester.givenName}{" "}
                          {order.requester.familyName}
                        </Text>
                      )}
                    </Group>
                  </Table.Td>
                  <Table.Td>
                    <Anchor
                      href={`/drinks/${order.drink.id}`}
                      underline="hover"
                    >
                      {order.drink.name}
                    </Anchor>
                  </Table.Td>
                  <Table.Td>
                    {new Date(order.requestedFor).toLocaleString()}
                  </Table.Td>
                  <Table.Td>
                    <Badge size="sm" color="indigo">
                      {order.status.charAt(0) +
                        order.status.slice(1).toLowerCase()}
                    </Badge>
                  </Table.Td>
                  <Table.Td>
                    {order.requester.email == email &&
                    !["CANCELLED", "DONE"].includes(order.status) ? (
                      <Group justify="center" wrap="nowrap" gap={"xs"}>
                        <ActionIcon
                          variant="transparent"
                          color="gray"
                          size="sm"
                          aria-label="actions"
                          onClick={() =>
                            cancelOrder({ variables: { id: order.id } })
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
    </>
  );
};

export default Orders;
