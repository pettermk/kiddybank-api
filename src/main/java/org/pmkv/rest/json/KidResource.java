package org.pmkv.rest.json;

import java.util.List;

import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.Response;
import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.persistence.EntityManager;
import javax.transaction.Transactional;

@Path("/kids")
@ApplicationScoped
public class KidResource {

    @Inject
    EntityManager entityManager;

    public KidResource() {
    }

    @GET
    public List<Kid> list() {
        return entityManager.createNamedQuery("kids.findAll", Kid.class)
        .getResultList();
    }

    @POST
    @Transactional
    public Response add(Kid kid) {
        entityManager.persist(kid);
        return Response.ok(kid).status(201).build();
    }

    @DELETE
    @Path("{id}")
    @Transactional
    public Response delete(Integer id) {
        Kid entity = entityManager.getReference(Kid.class, id);
        if (entity == null) {
            throw new WebApplicationException("Fruit with id of " + id + " does not exist.", 404);
        }
        entityManager.remove(entity);
        return Response.status(204).build();
    }
}