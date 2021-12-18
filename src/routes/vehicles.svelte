<script lang="ts">
  import { faCar, faMotorcycle, faPen, faPlus, faTimes } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { Button, ButtonGroup, Col, Column, Row, Table } from "sveltestrap";

  import Transition from "../components/Transition.svelte";
  import VehicleModal from "../components/VehicleModal.svelte";
  import { deleteVehicle } from "../mutation.js";
  import { getVehicles } from "../query.js";
  import { isVehicleModalOpen, vehicles } from "../store.js";

  const newVehicle = () => {
    return {
      id: undefined,
      model: "",
      vehicleType: 0,
      electric: false,
      horsepower: 0,
    };
  };

  let vehicle = newVehicle();

  const toggleVehicleModal = (object) => {
    vehicle = object;
    $isVehicleModalOpen = !$isVehicleModalOpen;
  };

  const onDelete = async (event, vehicle) => {
    event.preventDefault();
    await deleteVehicle(vehicle);
    await getVehicles();
  };
</script>

<Transition>
  <Table responsive rows={$vehicles} let:row class="align-middle">
    <Column width="2.5rem" class="text-center">
      {#if row.vehicleType == 0}
        <Fa icon={faCar} />
      {:else}
        <Fa icon={faMotorcycle} />
      {/if}
    </Column>
    <Column>
      {row.model}
    </Column>
    <Column width="5rem" class="text-center">
      <ButtonGroup>
        <Button color="dark" on:click={() => toggleVehicleModal(row)}>
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
      <Button color="dark" on:click={() => toggleVehicleModal(newVehicle())}>
        <Fa icon={faPlus} />
      </Button>
      <VehicleModal {vehicle} toggle={() => toggleVehicleModal(newVehicle())} />
    </Col>
  </Row>
</Transition>
