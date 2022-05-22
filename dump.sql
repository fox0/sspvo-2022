--
-- PostgreSQL database dump
--

-- Dumped from database version 14.3 (Ubuntu 14.3-1.pgdg20.04+1)
-- Dumped by pg_dump version 14.3 (Ubuntu 14.3-1.pgdg20.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: bef_ui_orgdirection(); Type: FUNCTION; Schema: public; Owner: fox
--

CREATE FUNCTION public.bef_ui_orgdirection() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  select
    xmlelement(name "PackageData",
      xmlelement(name "OrgDirection",
        xmlforest(
          new.id as "Uid",
          new.id_direction as "IdDirection"
    )))
  into new.xml;
  return new;
end;
$$;


ALTER FUNCTION public.bef_ui_orgdirection() OWNER TO fox;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: orgdirection; Type: TABLE; Schema: public; Owner: fox
--

CREATE TABLE public.orgdirection (
    id character varying(36) NOT NULL,
    id_direction integer NOT NULL,
    xml text NOT NULL
);


ALTER TABLE public.orgdirection OWNER TO fox;

--
-- Name: orgdirection orgdirection_pk; Type: CONSTRAINT; Schema: public; Owner: fox
--

ALTER TABLE ONLY public.orgdirection
    ADD CONSTRAINT orgdirection_pk PRIMARY KEY (id);


--
-- Name: orgdirection bef_ui_orgdirection; Type: TRIGGER; Schema: public; Owner: fox
--

CREATE TRIGGER bef_ui_orgdirection BEFORE INSERT OR UPDATE ON public.orgdirection FOR EACH ROW EXECUTE FUNCTION public.bef_ui_orgdirection();


--
-- PostgreSQL database dump complete
--
