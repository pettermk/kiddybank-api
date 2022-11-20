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

@Path("/transactions")
@ApplicationScoped
public class TransactionResource {

    @Inject
    EntityManager entityManager;

    public TransactionResource() {
    }

    @GET
    public List<Transaction> list() {
        return entityManager.createNamedQuery("transactions.findAll", Transaction.class)
        .getResultList();
    }

    @POST
    @Transactional
    public Response add(TransactionDTO transactionDto) {
        Kid kid = entityManager.getReference(
            Kid.class,
            transactionDto.kid_id);
        Date date = new Date();
        Transaction transaction = new Transaction(
            transactionDto.change,
            date,
            kid,
            transactionDto.description
        );
        Set<Transaction> transactions = kid.transactions;
        transactions.add(transaction);
        entityManager.persist(kid);
        entityManager.persist(transaction);

        return Response.ok(transaction).status(201).build();
    }

    @DELETE
    @Path("{id}")
    @Transactional
    public Response delete(Integer id) {
        Transaction entity = entityManager.getReference(Transaction.class, id);
        if (entity == null) {
            throw new WebApplicationException("Transaction with id of " + id + " does not exist.", 404);
        }
        entityManager.remove(entity);
        return Response.status(204).build();
    }
}
