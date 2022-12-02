package org.pmkv.rest.json;
import java.util.Set;
import java.util.HashSet;
import javax.persistence.CascadeType;
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.JoinColumn;
import javax.persistence.ManyToOne;
import javax.persistence.NamedQuery;
import javax.persistence.OneToMany;
import javax.persistence.QueryHint;
import javax.persistence.SequenceGenerator;
import com.fasterxml.jackson.annotation.JsonIgnore;

@Entity
@NamedQuery(
    name = "kids.findAll",
    query = "SELECT f FROM Kid f ORDER BY f.name",
    hints = @QueryHint(name = "org.hibernate.cacheable", value = "true")
)
public class Kid {
    @Id
    @SequenceGenerator(name = "kidSeq", sequenceName = "kid_id_seq", allocationSize = 1, initialValue = 1)
    @GeneratedValue(generator="kidSeq")
    public Long id;
    public String name;
    public double initial_balance;
    
    @JsonIgnore
    @ManyToOne
    @JoinColumn(name="kiddybankuser_id", nullable=false)
    public KiddyBankUser kiddybankuser;

    @JsonIgnore
    @OneToMany(mappedBy="kid", cascade = CascadeType.ALL)
    Set<Transaction> transactions = new HashSet<Transaction>();

    public Kid() {
    }

    public Kid(String name, double initial_balance) {
        this.name = name;
        this.initial_balance = initial_balance;
    }
}
