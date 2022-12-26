require 'cell'

describe GSPush::Cell do
  describe "interpolate_variables!" do
    let(:cell) { GSPush::Cell.new(value) }
    let(:variables) { { rownum: 1 } }
    before(:each) { cell.interpolate_variables!(variables) }

    subject { cell.value }

    context "with variables to interpolate" do
      let(:value) { "=2 + $$rownum" }
      it { should eq("=2 + 1") }

      context "when the same value needs to be interpolated multiple times" do
        let(:value) { "=$$rownum - $$rownum" }
        it { should eq("=1 - 1") }
      end

      context "when value is nil" do
        let(:value) { nil }
        it { should be_nil }
      end
    end

    context "with no variables to interpolate" do
      let(:value) { "test" }
      it { should eq(value) }
    end
  end
end
