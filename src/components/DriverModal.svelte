<script lang="ts">
  import { faCar, faFlagCheckered, faFont, faHome, faInfinity } from "@fortawesome/free-solid-svg-icons";
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

  import { addDriver, updateDriver } from "../mutation.js";
  import { getDrivers } from "../query.js";
  import { addresses, isDriverModalOpen, vehicles } from "../store.js";

  export let toggle = undefined;
  export let driver = undefined;

  let errorMessage = "";

  const setLimitDistance = (event, bool) => {
    event.preventDefault();
    driver.limitDistance = bool;
  };

  const onSubmit = async (event) => {
    event.preventDefault();
    try {
      if (driver.id) {
        await updateDriver(driver);
      } else {
        await addDriver(driver);
      }
      await getDrivers();
      $isDriverModalOpen = false;
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  };
</script>

<Modal isOpen={$isDriverModalOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Conducteur</ModalHeader>
  <ModalBody class="pb-0">
    <Form on:submit={onSubmit}>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faFont} />
          </InputGroupText>
          <Input type="text" name="name" id="name" placeholder="Nom" bind:value={driver.name} required />
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <ButtonGroup class="w-100">
          <Button
            block
            color={driver.limitDistance ? "dark" : "secondary"}
            id="limited"
            on:click={(event) => setLimitDistance(event, true)}
          >
            80
          </Button>
          <Tooltip target="limited" placement="top">Distance journalière limitée à 80 km</Tooltip>
          <Button
            block
            color={driver.limitDistance ? "secondary" : "dark"}
            id="unlimited"
            on:click={(event) => setLimitDistance(event, false)}
          >
            <Fa icon={faInfinity} />
          </Button>
          <Tooltip target="unlimited" placement="top">Distance journalière non limitée</Tooltip>
        </ButtonGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faCar} />
          </InputGroupText>
          <Input type="select" name="default-vehicle-id" id="default-vehicle-id" bind:value={driver.defaultVehicleId}>
            {#each $vehicles as vehicle}
              <option value={vehicle.id}>{vehicle.model}</option>
            {/each}
          </Input>
          <Tooltip target="default-vehicle-id" placement="top">Véhicule préféré</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faHome} />
          </InputGroupText>
          <Input type="select" name="default-from-id" id="default-from-id" bind:value={driver.defaultFromId}>
            {#each $addresses as address}
              {#if address.addressType == 0}
                <option value={address.id}>{address.title}</option>
              {/if}
            {/each}
          </Input>
          <Tooltip target="default-from-id" placement="top">Adresse personnelle préféré</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faFlagCheckered} />
          </InputGroupText>
          <Input type="select" name="default-to-id" id="default-to-id" bind:value={driver.defaultToId}>
            {#each $addresses as address}
              {#if address.addressType == 1}
                <option value={address.id}>{address.title}</option>
              {/if}
            {/each}
          </Input>
          <Tooltip target="default-to-id" placement="top">Adresse professionnelle préféré</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <Button block color="dark" type="submit" disabled={!driver.name}>Valider</Button>
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
