package org.pmkv.rest.json;
import java.util.Set;
import java.util.HashSet;
import javax.persistence.CascadeType;
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.NamedQuery;
import javax.persistence.OneToMany;
import javax.persistence.QueryHint;
import javax.persistence.SequenceGenerator;
import com.fasterxml.jackson.annotation.JsonIgnore;

@Entity
@NamedQuery(
    name = "users.findAll",
    query = "SELECT f FROM KiddyBankUser f ORDER BY f.username",
    hints = @QueryHint(name = "org.hibernate.cacheable", value = "true")
)
public class KiddyBankUser {
    @Id
    @SequenceGenerator(name = "userSeq", sequenceName = "user_id_seq", allocationSize = 1, initialValue = 1)
    @GeneratedValue(generator="userSeq")
    public Long id;
    public String username;
    public String external_id;

    @JsonIgnore
    @OneToMany(mappedBy="kiddybankuser", cascade = CascadeType.ALL)
    Set<Kid> kids = new HashSet<Kid>();

    public KiddyBankUser() {
    }

    public KiddyBankUser(String username, String external_id) {
        this.username = username;
        this.external_id = external_id;
    }
}
