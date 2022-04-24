<script lang="ts">
  import {
    faArrowRightArrowLeft,
    faArrowRightFromBracket,
    faArrowRightLong,
    faArrowRightToBracket,
    faCalendar,
    faCar,
    faGaugeHigh,
  } from "@fortawesome/free-solid-svg-icons";
  import { getMonth, getYear, parseISO } from "date-fns";
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

  import { createJourney, updateJourney } from "../mutation.js";
  import { getJourneys } from "../query.js";
  import { addresses, isJourneyModalOpen, journeyMonth, journeyYear, vehicles } from "../store.js";

  export let toggle = undefined;
  export let journey = undefined;
  export let kilometers = undefined;

  let errorMessage = "";

  $: journey.meters = kilometers * 1000;

  const setRoundTrip = (event, bool) => {
    event.preventDefault();
    journey.roundTrip = bool;
  };

  const onSubmit = async (event) => {
    event.preventDefault();
    try {
      if (journey.id) {
        await updateJourney(journey);
      } else {
        await createJourney(journey);
      }
      const date = parseISO(journey.date);
      const year = getYear(date);
      const month = getMonth(date) + 1;
      if ($journeyYear == year && $journeyMonth == month) {
        await getJourneys(journey.driverId, $journeyYear, $journeyMonth);
      } else {
        if ($journeyYear != year) {
          $journeyYear = year;
        }
        if ($journeyMonth != month) {
          $journeyMonth = month;
        }
      }
      $isJourneyModalOpen = false;
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  };
</script>

<Modal isOpen={$isJourneyModalOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Trajet</ModalHeader>
  <ModalBody class="pb-0">
    <Form on:submit={onSubmit}>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faCalendar} />
          </InputGroupText>
          <Input type="date" name="date" id="date" bind:value={journey.date} required />
          <Tooltip target="date" placement="top">Date</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faCar} />
          </InputGroupText>
          <Input type="select" name="vehicle-id" id="vehicle-id" bind:value={journey.vehicleId} required>
            {#each $vehicles as vehicle}
              <option value={vehicle.id}>{vehicle.model}</option>
            {/each}
          </Input>
          <Tooltip target="vehicle-id" placement="top">Véhicule</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faArrowRightFromBracket} />
          </InputGroupText>
          <Input type="select" name="from-id" id="from-id" bind:value={journey.fromId} required>
            {#each $addresses as address}
              <option value={address.id} disabled={address.id == journey.toId}>{address.title}</option>
            {/each}
          </Input>
          <Tooltip target="from-id" placement="top">Adresse de départ</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <ButtonGroup class="w-100">
          <Button
            block
            color={journey.roundTrip ? "secondary" : "dark"}
            id="oneway"
            on:click={(event) => setRoundTrip(event, false)}
          >
            <Fa icon={faArrowRightLong} />
          </Button>
          <Tooltip target="oneway" placement="top">Aller simple</Tooltip>
          <Button
            block
            color={journey.roundTrip ? "dark" : "secondary"}
            id="roundtrip"
            on:click={(event) => setRoundTrip(event, true)}
          >
            <Fa icon={faArrowRightArrowLeft} />
          </Button>
          <Tooltip target="roundtrip" placement="top">Aller retour</Tooltip>
        </ButtonGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faArrowRightToBracket} />
          </InputGroupText>
          <Input type="select" name="to-id" id="to-id" bind:value={journey.toId} required>
            {#each $addresses as address}
              <option value={address.id} disabled={address.id == journey.fromId}>{address.title}</option>
            {/each}
          </Input>
          <Tooltip target="to-id" placement="top">Adresse d'arrivée</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faGaugeHigh} />
          </InputGroupText>
          <Input type="number" name="kilometers" id="kilometers" bind:value={kilometers} required />
          <InputGroupText>km</InputGroupText>
          <Tooltip target="kilometers" placement="top">Distance en km</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <Button
          block
          color="dark"
          type="submit"
          disabled={!journey.driverId ||
            !journey.date ||
            !journey.vehicleId ||
            !journey.fromId ||
            !journey.toId ||
            !kilometers}>Valider</Button
        >
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
