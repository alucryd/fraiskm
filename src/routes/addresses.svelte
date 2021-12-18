<script lang="ts">
  import { faBriefcase, faHome, faPen, faPlus, faTimes } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { Button, ButtonGroup, Col, Column, Row, Table } from "sveltestrap";

  import AddressModal from "../components/AddressModal.svelte";
  import Transition from "../components/Transition.svelte";
  import { deleteAddress } from "../mutation.js";
  import { getAddresses } from "../query.js";
  import { addresses, isAddressModalOpen } from "../store.js";

  const newAddress = () => {
    return {
      id: undefined,
      title: "",
      label: "",
      addressType: 0,
    };
  };

  let address = newAddress();

  const toggleAddressModal = (object) => {
    address = object;
    $isAddressModalOpen = !$isAddressModalOpen;
  };

  const onDelete = async (event, address) => {
    event.preventDefault();
    await deleteAddress(address);
    await getAddresses();
  };
</script>

<Transition>
  <Table responsive rows={$addresses} let:row class="align-middle">
    <Column width="2.5rem" class="text-center">
      {#if row.addressType == 0}
        <Fa icon={faHome} />
      {:else}
        <Fa icon={faBriefcase} />
      {/if}
    </Column>
    <Column>
      {row.title}
    </Column>
    <Column width="5rem" class="text-center">
      <ButtonGroup>
        <Button color="dark" on:click={() => toggleAddressModal(row)}>
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
      <Button color="dark" on:click={() => toggleAddressModal(newAddress())}>
        <Fa icon={faPlus} />
      </Button>
      <AddressModal {address} toggle={() => toggleAddressModal(newAddress())} />
    </Col>
  </Row>
</Transition>
