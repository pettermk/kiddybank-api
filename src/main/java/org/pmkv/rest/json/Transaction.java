package org.pmkv.rest.json;
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.JoinColumn;
import javax.persistence.NamedQuery;
import javax.persistence.ManyToOne;
import javax.persistence.QueryHint;
import javax.persistence.SequenceGenerator;
import javax.persistence.Temporal;
import javax.persistence.TemporalType;
import com.fasterxml.jackson.annotation.JsonIgnore;

@Entity
@NamedQuery(
    name = "transactions.findAll",
    query = "SELECT f FROM Transaction f ORDER BY f.timestamp",
    hints = @QueryHint(name = "org.hibernate.cacheable", value = "true")
)
public class Transaction {
    @Id
    @SequenceGenerator(name = "transactionSeq", sequenceName = "transaction_id_seq", allocationSize = 1, initialValue = 1)
    @GeneratedValue(generator="transactionSeq")
    private Long id;
    @Temporal(TemporalType.TIMESTAMP)
    private java.util.Date timestamp;
    public double change;
    @JsonIgnore
    @ManyToOne
    @JoinColumn(name="kid_id", nullable=false)
    public Kid kid;
    public String description;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Transaction() {
    }

    public Transaction(
        double change,
        java.util.Date timstamp,
        Kid kid,
        String description
    ) {
        this.change = change;
        this.kid = kid;
        this.description = description;
    }
}
