package org.pmkv.rest.json;

import java.util.Date;
import java.util.List;
import java.util.Set;

import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.Response;

import org.hibernate.Session;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.persistence.EntityManager;
import javax.transaction.Transactional;

@Path("/users")
@ApplicationScoped
public class KiddyBankUserResource {

    @Inject
    EntityManager entityManager;

    public KiddyBankUserResource() {
    }

    @GET
    public List<KiddyBankUser> list() {
        return entityManager.createNamedQuery("users.findAll", KiddyBankUser.class)
        .getResultList();
    }

    @POST
    @Transactional
    public Response add(KiddyBankUser user) {
        entityManager.persist(user);
        return Response.ok(user).status(201).build();
    }

    @DELETE
    @Path("{id}")
    @Transactional
    public Response delete(Integer id) {
        KiddyBankUser entity = entityManager.getReference(KiddyBankUser.class, id);
        if (entity == null) {
            throw new WebApplicationException("Transaction with id of " + id + " does not exist.", 404);
        }
        entityManager.remove(entity);
        return Response.status(204).build();
    }
}
