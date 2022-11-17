package org.pmkv.rest.json;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.NamedQuery;
import javax.persistence.QueryHint;
import javax.persistence.SequenceGenerator;

@Entity
@NamedQuery(
    name = "kids.findAll",
    query = "SELECT f FROM Kid f ORDER BY f.name",
    hints = @QueryHint(name = "org.hibernate.cacheable", value = "true")
)
public class Kid {
    private Long id;
    public String name;
    public double initial_balance;

    @Id
    @SequenceGenerator(name = "kidSeq", sequenceName = "kid_id_seq", allocationSize = 1, initialValue = 1)
    @GeneratedValue(generator="kidSeq")
    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Kid() {
    }

    public Kid(String name, double initial_balance) {
        this.name = name;
        this.initial_balance = initial_balance;
    }
}
