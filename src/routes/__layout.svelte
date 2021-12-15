<script lang="ts">
  import {
    faCar,
    faCog,
    faKey,
    faMapMarkerAlt,
    faSignOutAlt,
    faTachometerAlt,
    faUsers,
  } from "@fortawesome/free-solid-svg-icons";
  import "bootstrap/dist/css/bootstrap.min.css";
  import Fa from "svelte-fa";
  import {
    Collapse,
    Container,
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownToggle,
    Nav,
    NavItem,
    NavLink,
    Navbar,
    NavbarBrand,
    NavbarToggler,
  } from "sveltestrap";

  import { user } from "../store.js";

  let isOpen = false;

  function handleUpdate(event) {
    isOpen = event.detail.isOpen;
  }
</script>

<Navbar color="dark" dark expand="md" class="sticky-top mb-3">
  <NavbarBrand href="/">
    <Fa icon={faTachometerAlt} />
    fraiskm
  </NavbarBrand>
  {#if $user}
    <NavbarToggler on:click={() => (isOpen = !isOpen)} />
    <Collapse {isOpen} navbar expand="md" on:update={handleUpdate}>
      <Nav navbar class="me-auto">
        <NavItem>
          <NavLink href="/drivers">
            <Fa icon={faUsers} />
            Mes conducteurs
          </NavLink>
        </NavItem>
        <NavItem>
          <NavLink href="/vehicles">
            <Fa icon={faCar} />
            Mes véhicules
          </NavLink>
        </NavItem>
        <NavItem>
          <NavLink href="/adresses">
            <Fa icon={faMapMarkerAlt} />
            Mes adresses
          </NavLink>
        </NavItem>
      </Nav>
      <Nav navbar class="ms-auto">
        <Dropdown nav inNavbar>
          <DropdownToggle nav>
            <Fa icon={faCog} />
          </DropdownToggle>
          <DropdownMenu end class="end-0">
            <DropdownItem>
              <Fa icon={faKey} />
              Mot de passe
            </DropdownItem>
            <DropdownItem divider />
            <DropdownItem>
              <Fa icon={faSignOutAlt} />
              Déconnexion
            </DropdownItem>
          </DropdownMenu>
        </Dropdown>
      </Nav>
    </Collapse>
  {/if}
</Navbar>

<Container fluid>
  <slot />
</Container>
