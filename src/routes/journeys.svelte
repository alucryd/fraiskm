<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    faArrowRightArrowLeft,
    faCalendar,
    faCalendarDay,
    faCar,
    faClone,
    faGaugeHigh,
    faLongArrowRight,
    faPen,
    faPlus,
    faSignsPost,
    faTimes,
    faUser,
  } from "@fortawesome/free-solid-svg-icons";
  import { format, parseISO } from "date-fns";
  import { range } from "lodash-es";
  import Fa from "svelte-fa";
  import { Button, ButtonGroup, Col, Column, Input, InputGroup, InputGroupText, Row, Table } from "sveltestrap";

  import JourneyDuplicateModal from "../components/JourneyDuplicateModal.svelte";
  import JourneyModal from "../components/JourneyModal.svelte";
  import Transition from "../components/Transition.svelte";
  import { deleteJourney } from "../mutation.js";
  import { getDistance, getJourneys } from "../query.js";
  import {
    addresses,
    currentDriverId,
    currentMonth,
    currentYear,
    drivers,
    getAddressById,
    getDriverById,
    getVehicleById,
    isJourneyDuplicateModalOpen,
    isJourneyModalOpen,
    journeys,
    ready,
    vehicles,
  } from "../store.js";

  const years = range(2021, new Date().getFullYear() + 1);
  const months = [
    [1, "Janvier"],
    [2, "Février"],
    [3, "Mars"],
    [4, "Avril"],
    [5, "Mai"],
    [6, "Juin"],
    [7, "Juillet"],
    [8, "Août"],
    [9, "Septembre"],
    [10, "Octobre"],
    [11, "Novembre"],
    [12, "Décembre"],
  ];

  const newJourney = async (driverId) => {
    const driver = getDriverById(driverId);
    return {
      id: undefined,
      driverId: driver.id,
      vehicleId: driver.defaultVehicleId,
      fromId: driver.defaultFromId,
      toId: driver.defaultToId,
      meters:
        driver.defaultFromId && driver.defaultToId ? await getDistance(driver.defaultFromId, driver.defaultToId) : 0,
      roundTrip: true,
    };
  };

  let journey = undefined;
  $: {
    journey = $currentDriverId ? newJourney($currentDriverId) : undefined;
  }
  $: (async () => $currentDriverId && (await getJourneys($currentDriverId, $currentYear, $currentMonth)))();

  const toggleJourneyModal = (object) => {
    journey = object;
    $isJourneyModalOpen = !$isJourneyModalOpen;
  };

  const toggleJourneyDuplicateModal = (object) => {
    journey = object;
    $isJourneyDuplicateModalOpen = !$isJourneyDuplicateModalOpen;
  };

  const onDelete = async (event, journey) => {
    event.preventDefault();
    await deleteJourney(journey);
    await getJourneys($currentDriverId, $currentYear, $currentMonth);
  };

  const addDriver = async (event) => {
    event.preventDefault();
    goto("/drivers", { replaceState: false });
  };

  const addVehicle = async (event) => {
    event.preventDefault();
    goto("/vehicles", { replaceState: false });
  };

  const addAddress = async (event) => {
    event.preventDefault();
    goto("/addresses", { replaceState: false });
  };
</script>

<Transition>
  {#if $ready}
    {#if !$vehicles.length || $addresses.length < 2 || !$drivers.length}
      <Row>
        {#if !$vehicles.length}
          <Col>
            <Button block color="dark" on:click={addVehicle}>Ajouter un véhicule</Button>
          </Col>
        {/if}
        {#if $addresses.length < 2}
          <Col>
            <Button block color="dark" on:click={addAddress}>Ajouter une adresse</Button>
          </Col>
        {/if}
        {#if !$drivers.length}
          <Col>
            <Button block color="dark" on:click={addDriver}>Ajouter un conducteur</Button>
          </Col>
        {/if}
      </Row>
    {:else}
      <Row>
        <Col>
          <InputGroup>
            <InputGroupText>
              <Fa icon={faUser} />
            </InputGroupText>
            <Input type="select" name="driver-id" id="driver-id" bind:value={$currentDriverId}>
              {#each $drivers as driver}
                <option value={driver.id}>{driver.name}</option>
              {/each}
            </Input>
          </InputGroup>
        </Col>
        <Col>
          <InputGroup>
            <InputGroupText>
              <Fa icon={faCalendar} />
            </InputGroupText>
            <Input type="select" name="year" id="year" bind:value={$currentYear}>
              {#each years as year}
                <option>{year}</option>
              {/each}
            </Input>
            <Input type="select" name="month" id="month" bind:value={$currentMonth}>
              {#each months as month}
                <option value={month[0]}>{month[1]}</option>
              {/each}
            </Input>
          </InputGroup>
        </Col>
      </Row>
      <Row class="mt-3">
        <Col>
          <Table responsive rows={$journeys} let:row class="align-middle">
            <Column width="2.5rem">
              <Fa icon={faCalendarDay} />
            </Column>
            <Column>
              {format(parseISO(row.date), "dd/MM/YYY")}
            </Column>
            <Column width="2.5rem">
              <Fa icon={faCar} />
            </Column>
            <Column>
              {getVehicleById(row.vehicleId).model}
            </Column>
            <Column width="2.5rem">
              <Fa icon={faSignsPost} />
            </Column>
            <Column>
              {getAddressById(row.fromId).title}
              {#if row.roundTrip}
                <Fa icon={faArrowRightArrowLeft} />
              {:else}
                <Fa icon={faLongArrowRight} />
              {/if}
              {getAddressById(row.toId).title}
            </Column>
            <Column width="2.5rem">
              <Fa icon={faGaugeHigh} />
            </Column>
            <Column>
              {row.meters / 1000} km
              {#if row.roundTrip}
                <sup>x2</sup>
              {/if}
            </Column>
            <Column width="7.5rem" class="text-center">
              <ButtonGroup>
                <Button color="dark" on:click={() => toggleJourneyModal(row)}>
                  <Fa icon={faPen} />
                </Button>
                <Button color="dark" on:click={() => toggleJourneyDuplicateModal(row)}>
                  <Fa icon={faClone} />
                </Button>
                <Button color="danger" on:click={(event) => onDelete(event, row)}>
                  <Fa icon={faTimes} />
                </Button>
              </ButtonGroup>
            </Column>
          </Table>
        </Col>
      </Row>
      {#if $currentDriverId}
        <Row class="flex-row-reverse">
          <Col class="flex-grow-0">
            <Button color="dark" on:click={async () => toggleJourneyModal(await newJourney($currentDriverId))}>
              <Fa icon={faPlus} />
            </Button>
          </Col>
        </Row>
        <JourneyModal
          {journey}
          toggle={() => toggleJourneyModal(newJourney($currentDriverId))}
          kilometers={journey.meters / 1000}
        />
        <JourneyDuplicateModal
          {journey}
          toggle={async () => toggleJourneyDuplicateModal(await newJourney($currentDriverId))}
        />
      {/if}
    {/if}
  {/if}
</Transition>
