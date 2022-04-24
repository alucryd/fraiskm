<script lang="ts">
  import { faCalendar } from "@fortawesome/free-solid-svg-icons";
  import { addDays, formatISO, getMonth, getYear, isBefore, isSameDay, parseISO } from "date-fns";
  import Fa from "svelte-fa";
  import {
    Button,
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

  import { createJourney } from "../mutation.js";
  import { getJourneys } from "../query.js";
  import { isJourneyDuplicateModalOpen, journeyMonth, journeyYear } from "../store.js";

  export let toggle = undefined;
  export let journey = undefined;

  let start = undefined;
  let end = undefined;

  let errorMessage = "";

  const onSubmit = async (event) => {
    event.preventDefault();
    try {
      start = parseISO(start);
      end = parseISO(end);
      if (isBefore(end, start)) {
        [start, end] = [end, start];
      }
      while (!isSameDay(start, end)) {
        journey.date = formatISO(start, { representation: "date" });
        await createJourney(journey);
        start = addDays(start, 1);
      }
      journey.date = formatISO(end, { representation: "date" });
      await createJourney(journey);
      const year = getYear(end);
      const month = getMonth(end) + 1;
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
      $isJourneyDuplicateModalOpen = false;
    } catch (error) {
      errorMessage = error.response.errors[0].message;
    }
  };
</script>

<Modal isOpen={$isJourneyDuplicateModalOpen} {toggle} class="text-center">
  <ModalHeader {toggle}>Duplication de trajet</ModalHeader>
  <ModalBody class="pb-0">
    <Form on:submit={onSubmit}>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faCalendar} />
          </InputGroupText>
          <Input type="date" name="start" id="start" bind:value={start} required />
          <Tooltip target="start" placement="top">Date de début</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <InputGroup>
          <InputGroupText>
            <Fa icon={faCalendar} />
          </InputGroupText>
          <Input type="date" name="end" id="end" bind:value={end} required />
          <Tooltip target="end" placement="top">Date de fin</Tooltip>
        </InputGroup>
      </FormGroup>
      <FormGroup>
        <Button block color="dark" type="submit" disabled={!start || !end}>Valider</Button>
      </FormGroup>
    </Form>
  </ModalBody>
</Modal>
