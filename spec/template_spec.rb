require 'template'

describe GSPush::Template do
  describe "#process!" do
    let(:template) { GSPush::Template.process!(input) }
    let(:input) { "foo,bar,baz" }

    it "creates rows" do
      expect(template.rows.length).to eq(1)
    end

    context "with cell modifiers" do
      let(:input) { "foo,[[align=right/format=bold]]bar,baz" }
      it "creates cells with the modifiers" do
        expect(template.rows[0].cells[1].modifier.align).to eq('right')
        expect(template.rows[0].cells[1].modifier.formats).to eq(['bold'])
      end
    end

    context "with cell modifiers with multiple values" do
      let(:input) { "foo,[[align=right/format=bold italic]]bar,baz" }
      it "creates cells with the modifiers" do
        expect(template.rows[0].cells[1].modifier.formats).to eq(['bold', 'italic'])
      end
    end

    context "with row modifiers" do
      let(:input) { "![[align=center/format=bold]]foo,bar,baz" }
      it "creates rows with the modifiers" do
        expect(template.rows[0].modifier.align).to eq('center')
        expect(template.rows[0].modifier.formats).to eq(['bold'])
      end

      describe "expand=" do
        let(:input) { "![[expand=2/format=bold]]foo,bar,baz\n![[expand]]" }

        it "expands the rows to SPREADSHEET_INFINITY" do
          expect(template.rows.length).to be(1000)
        end
      end
    end
  end
end
