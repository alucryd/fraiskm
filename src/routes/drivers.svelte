<script lang="ts">
  import { faPen, faPlus, faTimes, faUser } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { Button, ButtonGroup, Col, Column, Row, Table } from "sveltestrap";

  import DriverModal from "../components/DriverModal.svelte";
  import Transition from "../components/Transition.svelte";
  import { deleteDriver } from "../mutation.js";
  import { getDrivers } from "../query.js";
  import { drivers, isDriverModalOpen } from "../store.js";

  const newDriver = () => {
    return {
      id: undefined,
      name: "",
      limitDistance: true,
      defaultVehicleId: null,
      defaultFromId: null,
      defaultToId: null,
    };
  };

  let driver = newDriver();

  const toggleDriverModal = (object) => {
    driver = object;
    $isDriverModalOpen = !$isDriverModalOpen;
  };

  const onDelete = async (event, driver) => {
    event.preventDefault();
    await deleteDriver(driver);
    await getDrivers();
  };
</script>

<Transition>
  <Table responsive rows={$drivers} let:row class="align-middle">
    <Column width="2.5rem" class="text-center">
      <Fa icon={faUser} />
    </Column>
    <Column>
      {row.name}
    </Column>
    <Column width="5rem" class="text-center">
      <ButtonGroup>
        <Button color="dark" on:click={() => toggleDriverModal(row)}>
          <Fa icon={faPen} />
        </Button>
        <Button color="danger" on:click={(event) => onDelete(event, row)}>
          <Fa icon={faTimes} />
        </Button>
      </ButtonGroup>
    </Column>
  </Table>
  <Row class="flex-row-reverse">
    <Col class="flex-grow-0">
      <Button color="dark" on:click={() => toggleDriverModal(newDriver())}>
        <Fa icon={faPlus} />
      </Button>
      <DriverModal {driver} toggle={() => toggleDriverModal(newDriver())} />
    </Col>
  </Row>
</Transition>
