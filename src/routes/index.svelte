<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    faCalendar,
    faCar,
    faEuroSign,
    faGaugeHigh,
    faSquareRootVariable,
    faUser,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { Button, Col, Column, Input, InputGroup, InputGroupText, Row, Table } from "sveltestrap";

  import Transition from "../components/Transition.svelte";
  import { getTotals } from "../query.js";
  import {
    addresses,
    currentDriverId,
    currentYear,
    drivers,
    getVehicleById,
    ready,
    totals,
    vehicles,
    years,
  } from "../store.js";

  $: (async () => $currentDriverId && (await getTotals($currentDriverId, $currentYear)))();

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
          </InputGroup>
        </Col>
      </Row>
      <Row class="mt-3">
        <Col>
          <Table responsive rows={$totals} let:row class="align-middle">
            <Column width="2.5rem">
              <Fa icon={faCar} />
            </Column>
            <Column>
              {getVehicleById(row.vehicleId).model}
            </Column>
            <Column width="2.5rem">
              <Fa icon={faGaugeHigh} />
            </Column>
            <Column>
              {row.distance / 1000} km
            </Column>
            <Column width="2.5rem">
              <Fa icon={faSquareRootVariable} />
            </Column>
            <Column>
              {row.formula}
            </Column>
            <Column width="2.5rem">
              <Fa icon={faEuroSign} />
            </Column>
            <Column>
              {row.total}
            </Column>
          </Table>
        </Col>
      </Row>
    {/if}
  {/if}
</Transition>
