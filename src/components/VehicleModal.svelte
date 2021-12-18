<script lang="ts">
  import { faCar, faFont, faGasPump, faHorse, faMotorcycle, faPlug } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import {
    Button,
    ButtonGroup,
    Form,
    FormGroup,
    Input,
    InputGroup,
    InputGroupText,
    Modal,
    ModalBody,
    ModalHeader,
    Tooltip,
  } from "sveltestrap";

  import { addVehicle, updateVehicle } from "../mutation.js";
  import { getVehicles } from "../query.js";
  import { isVehicleModalOpen } from "../store.js";

  export let toggle = undefined;
  export let vehicle = undefined;

  let errorMessage = "";

  const setVehicleType = (event, int) => {
    event.preventDefault();
    vehicle.vehicleType = int;
  };

  const setElectric = (event, bool) => {
    event.preventDefault();
    vehicle.electric = bool;
  };

  const onSubmit = async (event) => {
    event.preventDefault();
    try {
      if (vehicle.id) {
        await updateVehicle(vehicle);
      } else {
        await addVehicle(vehicle);
      }
      await getVehicles();
      $isVehicleModalOpen = false;
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  };
</script>

<Modal isOpen={$isVehicleModalOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Véhicule</ModalHeader>
  <ModalBody class="pb-0">
    <Form on:submit={onSubmit}>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faFont} />
          </InputGroupText>
          <Input type="text" name="model" id="model" placeholder="Modèle" bind:value={vehicle.model} required />
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <ButtonGroup class="w-100">
          <Button
            block
            color={vehicle.vehicleType == 0 ? "dark" : "secondary"}
            id="vehicle-type-0"
            on:click={(event) => setVehicleType(event, 0)}
          >
            <Fa icon={faCar} />
          </Button>
          <Tooltip target="vehicle-type-0" placement="top">Voiture</Tooltip>
          <Button
            block
            color={vehicle.vehicleType == 1 ? "dark" : "secondary"}
            id="vehicle-type-1"
            on:click={(event) => setVehicleType(event, 1)}
          >
            <Fa icon={faMotorcycle} />
          </Button>
          <Tooltip target="vehicle-type-1" placement="top">Moto</Tooltip>
          <Button
            block
            color={vehicle.vehicleType == 2 ? "dark" : "secondary"}
            id="vehicle-type-2"
            on:click={(event) => setVehicleType(event, 2)}
          >
            <Fa icon={faMotorcycle} /> &lt; 50 cm<sup>3</sup>
            <Tooltip target="vehicle-type-1" placement="top"
              >Deux roues de cylindrée inférieure à 50 cm<sup>3</sup></Tooltip
            >
          </Button>
        </ButtonGroup>
      </FormGroup>
      <FormGroup>
        <ButtonGroup class="w-100">
          <Button
            block
            color={vehicle.electric ? "secondary" : "dark"}
            id="combustion"
            on:click={(event) => setElectric(event, false)}
          >
            <Fa icon={faGasPump} />
          </Button>
          <Tooltip target="combustion" placement="top">Thermique/Hybride</Tooltip>
          <Button
            block
            color={vehicle.electric ? "dark" : "secondary"}
            id="electric"
            on:click={(event) => setElectric(event, true)}
          >
            <Fa icon={faPlug} />
          </Button>
          <Tooltip target="electric" placement="top">Electrique</Tooltip>
        </ButtonGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faHorse} />
          </InputGroupText>
          <Input type="number" name="horsepower" id="horsepower" bind:value={vehicle.horsepower} required />
          <InputGroupText>ch</InputGroupText>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <Button block color="dark" type="submit" disabled={!vehicle.model || !vehicle.horsepower}>Valider</Button>
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
